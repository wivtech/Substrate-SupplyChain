// Make sure env variables are imported
if (!process.env.SENDGRID_API_KEY) {
  require('dotenv').config({ path: __dirname + '/.env' });
}

const crypto = require('crypto');
const moment = require('moment');
const sgMail = require('@sendgrid/mail');
const logger = require('../utils/logger');

const {
  SENDGRID_API_KEY,
  FRONTEND_HOST,
  JWT_ACCESS_ENCRYPTION,
  EMAIL_SENDER_ADDRESS,
  SENDGRID_VERIFICATION_EMAIL_ID,
  SENDGRID_FORGOT_PASSWORD_EMAIL_ID,
} = require('../constants');

if (!SENDGRID_API_KEY) {
  logger.error('No sendgrid key');
}

sgMail.setApiKey(SENDGRID_API_KEY);

// Send Verification Email
const SendVerificationEmail = (user, code) => {
  const unsubscribeCode = crypto.createHash('sha1').update(`${user.email}-${JWT_ACCESS_ENCRYPTION}`).digest('hex');
  const unsubscribeEmail = encodeURIComponent(user.email);

  // Email values
  const sentTime = moment().format('dddd, MMMM Do YYYY');
  const name = user.firstName || 'User';
  const verifyUrl = `${FRONTEND_HOST}/auth/verify-email/${code}`;
  const unsubscribeUrl = `${FRONTEND_HOST}/unsubscribe/${unsubscribeCode}/${unsubscribeEmail}`;

  return sgMail.send({
    'from': {
      'email': EMAIL_SENDER_ADDRESS,
    },
    'personalizations': [
      {
        'to': [{
          'email': user.email,
        }],
        'dynamic_template_data': {
          sentTime,
          name,
          verifyUrl,
          unsubscribeUrl,
        },
      },
    ],
    'template_id': SENDGRID_VERIFICATION_EMAIL_ID,
  });
};

// Forgot password Email
const SendForgotPasswordEmail = (user, code) => {
  const unsubscribeCode = crypto.createHash('sha1').update(`${user.email}-${JWT_ACCESS_ENCRYPTION}`).digest('hex');
  const unsubscribeEmail = encodeURIComponent(user.email);

  // Email values
  const sentTime = moment().format('dddd, MMMM Do YYYY');
  const name = user.firstName || 'User';
  const passwordResetUrl = `${FRONTEND_HOST}/auth/reset-password/${code}`;
  const unsubscribeUrl = `${FRONTEND_HOST}/unsubscribe/${unsubscribeCode}/${unsubscribeEmail}`;

  return sgMail.send({
    'from': {
      'email': EMAIL_SENDER_ADDRESS,
    },
    'personalizations': [
      {
        'to': [{
          'email': user.email,
        }],
        'dynamic_template_data': {
          sentTime,
          name,
          passwordResetUrl,
          unsubscribeUrl,
        },
      },
    ],
    'template_id': SENDGRID_FORGOT_PASSWORD_EMAIL_ID,
  });
};

module.exports = {
  SendVerificationEmail,
  SendForgotPasswordEmail,
};
