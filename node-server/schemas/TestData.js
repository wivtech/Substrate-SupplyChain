const Joi = require('../utils/validator2');

module.exports = {
  stringValue: Joi.string().label('StringValue').min(1).max(255).required(),
  numberValue: Joi.number().label('NumberValue').required(),
  booleanValue: Joi.boolean().label('BooleanValue').required(),
};