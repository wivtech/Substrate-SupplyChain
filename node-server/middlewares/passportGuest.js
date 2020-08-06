const passport = require('passport');

/**
 * Passport middleware, pass all requests, inject user if logged in, do nothing if guest user.
 * @param req
 * @param res
 * @param next
 */
const passportGuestMiddleware = (req, res, next) => {
  passport.authenticate('jwt', { session: false }, (err, user) => {
    if (err) { return next(err); }
    if (!user) { return next(); }

    req.logIn(user, (err) => {
      if (err) { return next(err); }
      return next();
    });
  })(req, res, next);
};

module.exports = passportGuestMiddleware;