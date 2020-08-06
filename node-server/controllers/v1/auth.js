const express = require('express');
const moment = require('moment');
const { to } = require('await-to-js');
const pick = require('lodash/pick');

const { User, EmailVerification, PasswordReset, JWT_ACCESS_ENCRYPTION } = require('../../models');
const userSchema = require('../../schemas/user');

const passportMiddleware = require('../../middlewares/passport');
const validateMiddleware = require('../../middlewares/validator2');
const Joi = require('../../utils/validator2');

const router = express.Router();

const { SendVerificationEmail, SendForgotPasswordEmail } = require('../../services/email');

// Register
router.post('/signup', validateMiddleware(userSchema), async (req, res) => {
  // Check if user tried signup before and not verified email
  let err, user;
  [err, user] = await to(User.findOne({ where: { email: req.body.email, verified: false } }));

  if (err) {
    return res.status(500).json({ msg: 'Sorry, can not sign up this time, please try again later.' }).end();
  }

  if (user) {
    // User record exists and not verified, just send verification email again
  } else {
    // User is new to system, create user record
    [err, user] = await to(User.create(pick(req.body, ['username', 'email', 'firstName', 'lastName', 'password'])));

    if (err || !user) {
      let errors = Joi.getPlainErrorsFromJsError(err);

      if (errors.username && (errors.username.indexOf('username must be unique') !== -1)) {
        errors['username'] = ['This username is already in use.'];
      }

      if (errors.email && (errors.email.indexOf('email must be unique') !== -1)) {
        errors['email'] = ['This email is already registered.'];
      }

      return res.status(422).json({ msg: 'Registration Failed!', errors }).end();
    }
  }

  // Send email to found/created user
  let emailVerification;
  [err, emailVerification] = await to(EmailVerification.create({
    userId: user.id,
    email: user.email,
    code: Math.random().toString(36).substring(2),
  }));

  if (err || !emailVerification) {
    return res.status(500).json({ msg: 'Sorry, can not sign up this time, please try again later.' }).end();
  }

  let result;
  [err, result] = await to(SendVerificationEmail(user, emailVerification.code));

  if (err) {
    return res.status(500).json({ msg: 'Sorry, can not send verification email this time, please try again later.' }).end();
  }

  res.json({ msg: 'We\'ve sent verification email, please follow instructions.' }).end();
});

// Resend Verification
router.post('/resend-verification-email', validateMiddleware(pick(userSchema, ['email'])), async (req, res) => {
  // Check if user with not verified status exists
  let err, user;
  [err, user] = await to(User.findOne({ where: { email: req.body.email, verified: false } }));
  if (err) {
    return res.status(422).json({ msg: 'Sorry, can not send verification now, please try again later.' }).end();
  }

  if (!user) {
    return res.status(422).json({ msg: 'There is no user with this email address.' }).end();
  }

  // Send new verification email
  let emailVerification;
  [err, emailVerification] = await to(EmailVerification.create({
    userId: user.id,
    email: user.email,
    code: Math.random().toString(36).substring(2),
  }));

  if (err || !emailVerification) {
    return res.status(500).json({ msg: 'Sorry, can not send verification now, please try again later.' }).end();
  }

  let result;
  [err, result] = await to(SendVerificationEmail(user, emailVerification.code));
  if (err) {
    return res.status(500).json({ msg: 'Sorry, can not send verification now, please try again later.' }).end();
  }

  res.json({ msg: 'Verification email resent!' });
});

// Verify email
router.post('/verify-email/:code', async (req, res) => {
  let err, emailVerification;
  [err, emailVerification] = await to(EmailVerification.findOne({
    where: {
      code: req.params.code,
      expired: false,
    },
  }));

  if (err || !emailVerification) {
    return res.status(404).json({ msg: 'Invalid verification token.' }).end();
  }

  // If token is older than 2 days, 2 * 24 * 3600 = 172800
  let timeDiff = moment().diff(emailVerification.createdAt, 'seconds');
  if (timeDiff >= 172800) {
    await to(emailVerification.update({ expired: true }));

    res.status(422).json({ msg: 'Verification email is older than 2 days and expired!' }).end();
    return;
  }

  // Find user for verification and make user verified
  let user;
  [err, user] = await to(User.findOne({ where: { id: emailVerification.userId } }));
  if (err || !user) {
    return res.status(404).json({ msg: 'There is no user with this verification code.' }).end();
  }

  let result;
  [err, result] = await to(user.update({ verified: true }));
  if (err) {
    return res.status(500).json({ msg: 'Sorry, can not verify user...' }).end();
  }

  // Make all verifications as expired
  [err, result] = await to(EmailVerification.update({ expired: true }, { where: { userId: user.id } }));
  if (err) {
    return res.status(500).json({ msg: 'There was an error while verifying user...' }).end();
  }

  return res.status(200).json({ msg: 'Email verification success!' });
});

// Login
router.post('/login', validateMiddleware(pick(userSchema, ['email', 'password'])), async (req, res) => {
  let err, user;
  [err, user] = await to(User.findOne({ where: { email: req.body.email } }));

  if (err || !user) {
    res.status(500).json({ msg: 'Invalid credential' }).end();
    return;
  }

  [err, user] = await to(user.comparePassword(req.body.password));

  if (err) {
    res.status(422).json({ msg: err.message || 'Invalid credential' }).end();
    return;
  }

  if (!user.verified) {
    res.status(401).json({ msg: 'User not verified. Please verify your email address.' }).end();
    return;
  }

  res.json({ msg: 'Login successful!', accessToken: user.getJwt() });
});

// GetMe
router.get('/me', passportMiddleware, async (req, res) => {
  res.json(pick(req.user, [
    'name',
    'email',
    'firstName',
    'lastName',
    'role',
  ]));
});

// Request Password Reset
router.post('/request-password-reset', validateMiddleware(pick(userSchema, ['email'])), async (req, res) => {
  // Check if user exists with the email
  let err, user;
  [err, user] = await to(User.findOne({ where: { email: req.body.email } }));
  if (err || !user) {
    res.status(422).json({ msg: 'There is no user with this email.' }).end();
    return;
  }

  if (!user.verified) {
    res.status(401).json({ msg: 'User did not verified email yet.' }).end();
    return;
  }

  // Create Password Reset token, send email, send status to front end
  let passwordReset;
  [err, passwordReset] = await to(PasswordReset.create({
    userId: user.id,
    email: user.email,
    code: Math.random().toString(36).substring(2) + Math.random().toString(36).substring(2),
  }));
  if (err || !passwordReset) {
    res.status(500).json({ msg: 'Sorry, can not reset password at this time, try again later.' }).end();
    return;
  }

  let emailResult;
  [err, emailResult] = await to(SendForgotPasswordEmail(user, passwordReset.code));
  if (err) {
    res.status(500).json({ msg: 'Sorry, can not send reset password email at this time, try again later.' }).end();
    return;
  }

  res.status(200).json({ msg: 'We\'ve sent instruction to your email. Please follow it to reset your password.' }).end();
});

// Verify Password Reset token
router.post('/verify-password-reset-token', async (req, res) => {
  if (!req.body.token) {
    res.status(422).json({ msg: 'Invalid request!' }).end();
    return;
  }

  let err, passwordReset;
  [err, passwordReset] = await to(PasswordReset.findOne({ where: { code: req.body.token, expired: false } }));
  if (err || !passwordReset || passwordReset.expired) {
    res.status(422).json({ msg: 'Invalid request!' }).end();
    return;
  }

  // If token is older than 2 days, 2 * 24 * 3600 = 172800
  let timeDiff = moment().diff(passwordReset.createdAt, 'seconds');
  if (timeDiff >= 172800) {
    await to(passwordReset.update({ expired: true }));

    res.status(422).json({ msg: 'Request expired!' }).end();
    return;
  }

  res.status(200).json({ msg: 'Valid token' }).end();
});

// Reset Password
router.post('/reset-password', validateMiddleware(pick(userSchema, ['password'])), async (req, res) => {
  if (!req.body.token) {
    res.status(422).json({ msg: 'Invalid request!' }).end();
    return;
  }

  // Find token
  let err, passwordReset;
  [err, passwordReset] = await to(PasswordReset.findOne({ where: { code: req.body.token, expired: false } }));
  if (err || !passwordReset || passwordReset.expired) {
    res.status(422).json({ msg: 'Invalid request!' }).end();
    return;
  }

  // If token is older than 2 days, 2 * 24 * 3600 = 172800
  let timeDiff = moment().diff(passwordReset.createdAt, 'seconds');
  if (timeDiff >= 172800) {
    await to(passwordReset.update({ expired: true }));

    res.status(422).json({ msg: 'Request expired!' }).end();
    return;
  }

  // Find user, Update password
  let user;
  [err, user] = await to(User.findOne({ where: { email: passwordReset.email } }));
  if (err || !user) {
    res.status(422).json({ msg: 'No user found!' }).end();
    return;
  }

  [err] = await to(user.update({ password: req.body.password }));
  if (err) {
    res.status(500).json({ msg: 'Error occurred while resetting password.' }).end();
    return;
  }

  // Expire all requests
  await to(PasswordReset.update({ expired: true }, { where: { userId: user.id } }));

  res.status(200).json({ msg: 'Password reset successful.' }).end();
});

router.post('/unsubscribe', async (req, res) => {
  let errors = Joi.validateToPlainErrors(req.body, {
    'unsubscribeCode': Joi.string().label('Code').required(),
    'unsubscribeEmail': Joi.string().label('Email').required(),
  });

  if (Joi.hasPlainError(errors)) {
    return res.status(422).json({ errors, msg: 'Please enter correct values' }).end();
  }

  // Check if valid unsubscribe
  const email = decodeURIComponent(req.body.unsubscribeEmail);
  const codeCalculated = crypto.createHash('sha1').update(`${email}-${JWT_ACCESS_ENCRYPTION}`).digest('hex');

  if (codeCalculated !== req.body.unsubscribeCode) {
    return res.status(422).json({ msg: 'Invalid request' }).end();
  }

  let err, user;
  [err, user] = await to(User.findOne({ where: { email } }));
  if (err || !user) {
    return res.status(422).json({ msg: 'There is no user with this email' }).end();
  }

  [err] = await to(user.update({ unsubscribed: true }));
  if (err) {
    return res.status(422).json({ msg: 'Sorry, error occurred while unsubscribing' }).end();
  }

  return res.json({ msg: 'Successfully unsubscribed' }).end();
});

module.exports = router;