'use strict';
module.exports = (sequelize, DataTypes) => {
  const EmailVerification = sequelize.define('EmailVerification', {
    userId: DataTypes.INTEGER,
    email: DataTypes.STRING,
    code: DataTypes.STRING,
    expired: DataTypes.BOOLEAN
  }, {});
  EmailVerification.associate = function(models) {
    // associations can be defined here
  };
  return EmailVerification;
};