const { ExtractJwt, Strategy } = require('passport-jwt');
const { User } = require('../models');
const { to } = require('await-to-js');
const { JWT_ACCESS_ENCRYPTION } = require('../constants');

module.exports = (passport) => {
  const options = {
    jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
    secretOrKey: JWT_ACCESS_ENCRYPTION,
  };

  // Use jwt strategy
  passport.use(new Strategy(options, async function (jwtPayload, done) {
    // Verify callback

    let err, user;
    [err, user] = await to(User.findById(jwtPayload.userId));

    if (err) {
      return done(err, false);
    }

    if (user) {
      return done(null, user);
    } else {
      return done(null, false);
    }
  }));

  passport.serializeUser(function (user, done) {
    done(null, user);
  });

  passport.deserializeUser(function (user, done) {
    done(null, user);
  });
};