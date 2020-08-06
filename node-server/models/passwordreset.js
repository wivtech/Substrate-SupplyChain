'use strict';
module.exports = (sequelize, DataTypes) => {
  const PasswordReset = sequelize.define('PasswordReset', {
    userId: DataTypes.INTEGER,
    email: DataTypes.STRING,
    code: DataTypes.STRING,
    expired: DataTypes.BOOLEAN
  }, {});
  PasswordReset.associate = function(models) {
    // associations can be defined here
  };
  return PasswordReset;
};