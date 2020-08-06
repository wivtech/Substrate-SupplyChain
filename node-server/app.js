//Init env
require('dotenv').config();

// Import modules
const express = require('express');
const bodyParser = require('body-parser');
const passport = require('passport');
const configPassport = require('./config/passport');
const morgan = require('morgan');
const { to } = require('await-to-js');
const api = require('./services/api');

const logger = require('./utils/logger');

// Constants
const { PORT } = require('./constants');

// Initialize express
const app = express();
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extend: true }));
app.disable('x-powered-by');

// Initialize passport
configPassport(passport);
app.use(passport.initialize());

// Logger middleware
app.use((req, res, next) => {
  logger.log(`[${req.method} ${req.originalUrl}] > ${req.headers['x-forwarded-for'] || req.connection.remoteAddress}`);
  next();
});
app.use(morgan('short'));

// CORS middleware
app.use((req, res, next) => {
  // const ALLOWED_ORIGINS = [
  //   'http://my-lovely-site.com',
  //   'https://my-lovely-site.com',
  // ];
  // if (ALLOWED_ORIGINS.indexOf(req.get('origin')) === -1) {
  //   res.status(403).end();
  //   return;
  // }

  res.header('Access-Control-Allow-Origin', req.get('origin'));
  res.header('Access-Control-Allow-Methods', 'OPTIONS,GET,POST,PUT,DELETE');
  res.header('Access-Control-Allow-Headers', 'Content-Type, Authorization, x-requested-with');

  // intercept OPTIONS method
  if (req.method === 'OPTIONS') {
    res.sendStatus(200);
  } else {
    next();
  }
});

// Controllers
const MaintainController = require('./controllers/maintain');
const V1Controller = require('./controllers/v1');

// Main
(async function main () {
  // Get current server's public ip address
  let [errGetIp, ip] = await to(api.utils.getPublicIp());
  if (!errGetIp) {
    logger.log(`Server public IP: ${ip}`);
  }

  // Route definitions
  app.get('/', (req, res) => {
    res.json({ msg: 'express server' }).end();
  });

  // Inspection endpoint
  app.use('/maintain', MaintainController);

  // Versioned endpoints
  app.use('/v1', V1Controller);

  // Start instance on port
  app.listen(PORT, () => {
    logger.log(`Server started on port: ${PORT}`);
  });
})();