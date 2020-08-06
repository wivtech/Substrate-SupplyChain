const Joi = require('joi');

// Validator2 based on JOI
/**
 *   constructor (props) {
 *     super(props);
 *
 *     // Add errors object to state
 *     this.state = {
 *       errors: {}
 *     };
 *
 *     // Init schema
 *     this.schema = {
 *       'name': Joi.string().label('Name').min(3).max(255)required(),
 *       'age': Joi.number().label('Age').positive().required(),
 *     };
 *
 *   }
 *
 *   // On change handler, do individual validation
 *   handleChange (val, key) {
 *     this.setState({
 *       errors: {
 *         ...this.state.errors,
 *         [key]: Joi.validateToPlainErrors(val, this.schema[key])
 *       }
 *     });
 *
 *     this.props.editForm({
 *       [key]: val
 *     });
 *   }
 *
 *   handleBlur () {
 *     // Validate all fields
 *     this.setState({
 *       errors: Joi.validateToPlainErrors(this.props.form, this.schema)
 *     });
 *   }
 *
 *   // Do all validation on submit
 *   onSubmit () {
 *     const errors = Joi.validateToPlainErrors(this.props.form, this.schema);
 *     this.setState({
 *       errors
 *     });
 *
 *     if (Joi.hasPlainError(errors)) {
 *       notifier.error('Please fix errors');
 *       return;
 *     }
 *
 *     // ... Do submit
 *   }
 *
 *   // Display error
 *   render() {
 *     const {errors} = this.state;
 *
 *     return (
 *       // ...
 *       hasError={Joi.getFirstPlainError(errors, 'name')}  // Same method for has error and first error logic
 *       error={Joi.getFirstPlainError(errors, 'name')}
 *       // ...
 *     );
 *   }
 */

// Validate and return error in PlainError
// object validation => {name: 'error1', age: 'error2'}
// single value validation => ['error1']
Joi.__proto__.validateToPlainErrors = function (val, schema) {
  if (!schema) {
    return null;
  }

  const result = Joi.validate(val, schema, {
    abortEarly: false,
    allowUnknown: true,
  });

  if (typeof val === 'object') {
    let errors = {};
    if (result.error && result.error.details && result.error.details.length) {
      for (let i = 0; i < result.error.details.length; i++) {
        const key = result.error.details[i].path[0];
        const msg = result.error.details[i].message;

        if (!errors[key]) {
          errors[key] = [msg];
        } else {
          errors[key].push(msg);
        }
      }
    }
    return errors;
  } else {
    let error = [];
    if (result.error && result.error.details && result.error.details.length) {
      for (let i = 0; i < result.error.details.length; i++) {
        error.push(result.error.details[i].message);
      }
    }
    return error.length ? error : null;
  }
};

// Check if object contains error
// {} => false
// {name: null} => false
// {name: 'error1'} => true
// {name: ['error1']} => true
// [] = false
// ['error1'] = true
Joi.__proto__.hasPlainError = function (error) {
  if (typeof error === 'object') {
    let hasError = false;

    for (let key in error) {
      if (error.hasOwnProperty(key)) {
        if (error[key]) {
          if (Array.isArray(error[key])) {
            for (let i = 0; i < error[key].length; i++) {
              if (error[key][i]) {
                hasError = true;
                break;
              }
            }
          } else if (typeof error[key] === 'string') {
            hasError = true;
          }
        }
      }
      if (hasError) {
        break;
      }
    }

    return hasError;
  } else {
    return false;
  }
};

// Get one field's error from plain error object, return null if no error
// ex: const error = {name: ['error1', 'error2']}
//     getFirstPlainError(error, 'name') => 'error1'
Joi.__proto__.getFirstPlainError = function (error, key) {
  if (typeof error === 'object') {
    if (error && error.hasOwnProperty(key)) {
      if (Array.isArray(error[key])) {
        if (error[key].length) {
          return error[key][0];
        } else {
          return null;
        }
      } else {
        return error[key];
      }
    } else {
      return null;
    }
  } else {
    return null;
  }
};

Joi.__proto__.getPlainErrorsFromJsError = (err) => {
  let errors = {};
  if (err && err.errors && err.errors.length) {
    for (let i = 0; i < err.errors.length; i++) {
      const key = err.errors[i].path;
      const msg = err.errors[i].message;

      if (!errors[key]) {
        errors[key] = [msg];
      } else {
        errors[key].push(msg);
      }
    }
  }
  return errors;
};

module.exports = Joi;