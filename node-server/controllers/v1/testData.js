const express = require('express');
const pick = require('lodash/pick');
const { to } = require('await-to-js');
const passportMiddleware = require('../../middlewares/passport');
const validateMiddleware = require('../../middlewares/validator2');
const { TestData } = require('../../models');
const testDataSchema = require('../../schemas/TestData');
const router = express.Router();

/**
 * Create one with data
 */
router.post('/', passportMiddleware, validateMiddleware(testDataSchema), async (req, res) => {
  const dataInput = pick(req.body, [
    'stringValue',
    'numberValue',
    'booleanValue',
  ]);

  let [errCreate, record] = await to(TestData.create(dataInput));
  if (errCreate || !record) {
    return res.status(500).json({ msg: 'Can not save data' }).end();
  }

  return res.json({ msg: 'Saved successfully', data: record }).end();
});

/**
 * Get one by id
 */
router.get('/:id', passportMiddleware, async (req, res) => {
  const id = req.params.id;

  let [errFind, record] = await to(TestData.findOne({ where: { id } }));
  if (errFind) {
    return res.status(500).json({ msg: `Can not find data with id: ${id}` }).end();
  }

  if (!record) {
    return res.status(404).json({ msg: `There is no record with id: ${id}` }).end();
  }

  return res.json({ msg: `Get successful`, data: record }).end();
});

/**
 * Update with id, data
 */
router.put('/:id', passportMiddleware, validateMiddleware(testDataSchema), async (req, res) => {
  const id = req.params.id;
  const dataInput = pick(req.body, [
    'stringValue',
    'numberValue',
    'booleanValue',
  ]);

  let [errFind, record] = await to(TestData.findOne({ where: { id } }));
  if (errFind || !record) {
    return res.status(404).json({ msg: 'Can not find data' }).end();
  }

  let [errUpdate, recordUpdated] = await to(record.update(dataInput));
  if (errUpdate) {
    return res.status(500).json({ msg: 'Can not update data' }).end();
  }

  return res.json({ msg: 'Saved successfully', data: record }).end();
});

/**
 * Get list, paginated
 */
router.get('/', passportMiddleware, async (req, res) => {
  let current = Number.parseInt(req.query.current || 1);
  const pageSize = Number.parseInt(req.query.pageSize || 50);
  const sorterField = req.query.sorterField || false;
  const sorterOrder = req.query.sorterOrder || false;

  current = current || 1;

  try {
    let findOptions = {
      offset: (current - 1) * pageSize,
      limit: pageSize,
      attributes: ['id', 'stringValue', 'numberValue', 'booleanValue', 'createdAt', 'updatedAt'],
    };

    if (sorterField && sorterOrder) {
      findOptions.order = [[sorterField, (sorterOrder === 'ascend' ? 'ASC' : 'DESC')]];
    }

    const results = await TestData.findAndCountAll(findOptions);

    res.status(200).json({ items: results.rows, current, total: results.count });
  } catch (e) {
    res.status(500).json({
      msg: `Sorry, Error occurred during fetching data`,
    });
  }
});

/**
 * Delete by id
 */
router.delete('/:id', passportMiddleware, async (req, res) => {
  const id = req.params.id;

  let [errDelete] = await to(TestData.destroy({ where: { id } }));
  if (errDelete) {
    return res.status(500).json({ msg: `Can not delete data` }).end();
  }

  return res.json({ msg: `Delete successful` }).end();
});

module.exports = router;
