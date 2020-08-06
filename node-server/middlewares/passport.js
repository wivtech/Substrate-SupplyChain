const passport = require('passport');

/**
 * Passport middleware, pass only authenticated requests.
 * @param req
 * @param res
 * @param next
 */
const passportMiddleware = (req, res, next) => {
  passport.authenticate('jwt', { session: false }, (err, user) => {
    if (err) { return next(err); }
    if (!user) { return res.status(401).json({ msg: 'Invalid Token, Please log in!' }); }

    if (!user.verified) {
      return res.status(401).json({ msg: 'User not verified. Please verify your email address.' });
    }

    req.logIn(user, (err) => {
      if (err) { return next(err); }
      return next();
    });
  })(req, res, next);
};

module.exports = passportMiddleware;