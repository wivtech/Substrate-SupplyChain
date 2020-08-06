'use strict';
const bcrypt = require('bcrypt');
const { to } = require('await-to-js');
const jwt = require('jsonwebtoken');
const { JWT_ACCESS_EXPIRATION, JWT_ACCESS_ENCRYPTION } = require('../constants');

module.exports = (sequelize, DataTypes) => {
  const User = sequelize.define('User', {
    username: DataTypes.STRING,
    firstName: DataTypes.STRING,
    lastName: DataTypes.STRING,
    email: DataTypes.STRING,
    role: DataTypes.STRING,
    verified: DataTypes.BOOLEAN,
    password: DataTypes.STRING,
    unsubscribed: DataTypes.BOOLEAN,
  }, {});

  User.associate = function (models) {
    // associations can be defined here
  };

  User.beforeSave(async function (user, options) {
    if (user.changed('password')) {
      let [err, hash] = await to(bcrypt.hash(user.password, 10));

      if (err) {
        throw err;
      }

      user.password = hash;
    }
  });

  User.prototype.comparePassword = async function (password) {
    if (!this.password) {
      throw Error('User does not have password');
    }

    let [err, pass] = await to(bcrypt.compare(password, this.password));

    if (err || !pass) {
      throw Error('Invalid password!');
    }

    return this;
  };

  User.prototype.getJwt = function () {
    return `Bearer ${jwt.sign({ userId: this.id }, JWT_ACCESS_ENCRYPTION, { expiresIn: JWT_ACCESS_EXPIRATION })}`;
  };

  return User;
};