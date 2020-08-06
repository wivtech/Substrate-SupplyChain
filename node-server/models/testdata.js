'use strict';
module.exports = (sequelize, DataTypes) => {
  const TestData = sequelize.define('TestData', {
    stringValue: DataTypes.STRING,
    numberValue: DataTypes.INTEGER,
    booleanValue: DataTypes.BOOLEAN
  }, {});
  TestData.associate = function(models) {
    // associations can be defined here
  };
  return TestData;
};