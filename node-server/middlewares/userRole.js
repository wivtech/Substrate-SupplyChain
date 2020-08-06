/**
 * User role checking middleware, passes certain user roles only.
 * @param role - User role to pass, empty means anyone can access.
 * @returns {Function}
 */
const userRoleMiddleware = (role) => (req, res, next) => {
  if (!role) {
    return next();
  }

  if (!req.user || (req.user.role !== role)) {
    return res.status(401).json({ msg: 'Invalid user permission.' }).end();
  }

  return next();
};

module.exports = userRoleMiddleware;