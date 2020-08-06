const express = require('express');
const { to } = require('await-to-js');
const git = require('git-last-commit');
const db = require('../../models');
const { DUMMY_VALUE_FOR_TEST } = require('../../constants');

const router = express.Router();

router.get('/', (req, res) => {
  res.json({ msg: 'Bonjour | Hallo | Hello | Ciao | 你好 | Zdravo | Saluton | Привет' });
});

router.get('/inspect', async (req, res) => {
  // get last migration
  let [errGetMigration, migrations] = await to(db.sequelize.query('SELECT * FROM `SequelizeMeta` ORDER BY name DESC LIMIT 1', { type: db.sequelize.QueryTypes.SELECT }));
  const lastMigration = (!errGetMigration && migrations && migrations.length && migrations[0].name) || 'No migration';

  // get last commit
  git.getLastCommit((errGetLastCommit, commit) => {
    // read commit object properties
    let lastDevName = '';
    let lastDevEmail = '';
    let lastCommitMsg = '';
    let lastCommitTime = '';
    let lastCommitHash = '';

    if (!errGetLastCommit && commit) {
      lastDevName = (commit.committer && commit.committer.name) || '';
      lastDevEmail = (commit.committer && commit.committer.email) || '';
      lastCommitMsg = commit.subject || '';
      lastCommitTime = commit.committedOn || '';
      lastCommitHash = commit.hash || '';
    }

    res.json({
      msg: 'Developer inspection',
      data: {
        dbMigration: {
          lastMigration,
        },
        gitCommit: {
          lastDevName,
          lastDevEmail,
          lastCommitMsg,
          lastCommitTime,
          lastCommitHash,
        },
        env: {
          DUMMY_VALUE_FOR_TEST,
        },
      },
    });
  });
});

module.exports = router;