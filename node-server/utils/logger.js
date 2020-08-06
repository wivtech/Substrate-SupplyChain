const moment = require('moment');

const LOGGER_MOMENT_FORMAT = 'YYYY-MM-DD HH:mm:ss';

let lastLevel = 0;

module.exports = {
  error: (string, level = 0, ...rest) => {
    if (lastLevel !== level) {
      let separator = ' ';
      for (let i = 0; i < lastLevel; i++) {
        separator += ' ';
      }
      separator += '+--------------------------';
      console.log(separator);
    }

    let spaces = '';
    for (let i = 0; i < level; i++) {
      spaces += ' ';
    }

    console.error(`${spaces} |-x- [${moment().format(LOGGER_MOMENT_FORMAT)}] ${string}`, ...rest);
    lastLevel = level;
  },
  log: (string, level = 0, ...rest) => {
    if (lastLevel !== level) {
      let separator = ' ';
      for (let i = 0; i < lastLevel; i++) {
        separator += ' ';
      }
      separator += '+--------------------------';
      console.log(separator);
    }

    let spaces = '';
    for (let i = 0; i < level; i++) {
      spaces += ' ';
    }

    console.log(`${spaces} |--- [${moment().format(LOGGER_MOMENT_FORMAT)}] ${string}`, ...rest);
    lastLevel = level;
  }
};
