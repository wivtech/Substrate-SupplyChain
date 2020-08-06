const Axios = require('axios');
const https = require('https');

const axios = Axios.create({
  httpsAgent: new https.Agent({
    rejectUnauthorized: false,
  }),
});

module.exports = {
  utils: {
    getPublicIp: () => axios.get('https://ipinfo.io/ip')
      .then(res => res.data),
  },
};
