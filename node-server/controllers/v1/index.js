const express = require('express');
const router = express.Router();

const AuthController = require('./auth');
const TestDataController = require('./testData');

router.use('/auth', AuthController);
router.use('/test-data', TestDataController);

module.exports = router;