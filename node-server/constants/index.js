module.exports = {
  DUMMY_VALUE_FOR_TEST: process.env.DUMMY_VALUE_FOR_TEST || '',

  PORT: Number.parseInt(process.env.PORT || 3100),
  FRONTEND_HOST: (process.env.FRONTEND_HOST && process.env.FRONTEND_HOST.startsWith('http'))
    ? process.env.FRONTEND_HOST
    : ('https://' + process.env.FRONTEND_HOST),

  JWT_ACCESS_EXPIRATION: Number.parseInt(process.env.JWT_ACCESS_EXPIRATION || 3600),
  JWT_ACCESS_ENCRYPTION: process.env.JWT_ACCESS_ENCRYPTION || 'Super Secret',

  EMAIL_SENDER_ADDRESS: process.env.EMAIL_SENDER_ADDRESS || 'info@com.com',
  SENDGRID_API_KEY: process.env.SENDGRID_API_KEY || '',
  SENDGRID_VERIFICATION_EMAIL_ID: process.env.SENDGRID_VERIFICATION_EMAIL_ID || '',
  SENDGRID_FORGOT_PASSWORD_EMAIL_ID: process.env.SENDGRID_FORGOT_PASSWORD_EMAIL_ID || '',
};