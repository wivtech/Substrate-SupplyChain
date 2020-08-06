const Joi = require('../utils/validator2');

module.exports = {
  username: Joi.string().label('Username').min(1).max(255).required(),
  firstName: Joi.string().label('First Name').min(1).max(255).required(),
  lastName: Joi.string().label('Last Name').min(1).max(255).required(),
  email: Joi.string().label('Email').email().required(),
  password: Joi.string().label('Password').min(6).max(255).required(),
};