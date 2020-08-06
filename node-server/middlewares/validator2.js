const Joi = require('../utils/validator2');

/**
 * Input validation middleware, using Joi
 * @param schema - Schema to do validation
 * @returns {Function}
 */
const validate = (schema) => (req, res, next) => {
  let errors = Joi.validateToPlainErrors(req.body, schema);

  if (Joi.hasPlainError(errors)) {
    res.status(422).json({ errors, msg: 'Please enter correct values.' }).end();
    return;
  }

  next();
};

module.exports = validate;