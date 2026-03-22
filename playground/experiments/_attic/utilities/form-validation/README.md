# Form Validation Library

A comprehensive, chainable form validation library for JavaScript with support for sync/async validation, cross-field validation, and conditional rules.

## Features

- **Chainable API**: Fluent interface for defining validation rules
- **Built-in Validators**: Email, URL, phone, credit card, etc.
- **Custom Validators**: Add your own validation logic
- **Async Validation**: Support for asynchronous validators (API calls, database lookups)
- **Cross-field Validation**: Validate fields based on other field values
- **Conditional Validation**: Apply rules conditionally
- **Detailed Error Messages**: Custom, descriptive error messages
- **Zero Dependencies**: Pure JavaScript, no external libraries

## Installation

```bash
# Copy validator.js to your project
cp validator.js your-project/
```

## Quick Start

```javascript
const { FormValidator } = require('./validator');

// Create validator
const validator = new FormValidator();

// Define rules
validator.field('email')
    .required()
    .email();

validator.field('password')
    .required()
    .minLength(8);

// Validate
const result = await validator.validate({
    email: 'user@example.com',
    password: 'securepass123'
});

if (result.isValid) {
    console.log('Form is valid!');
} else {
    console.log('Errors:', result.errors);
}
```

## Built-in Validators

### Required
```javascript
validator.field('username').required('Username is required');
```

### String Length
```javascript
validator.field('username')
    .minLength(3, 'At least 3 characters')
    .maxLength(20, 'At most 20 characters');
```

### Numeric Range
```javascript
validator.field('age')
    .min(18, 'Must be 18 or older')
    .max(100, 'Invalid age');
```

### Email
```javascript
validator.field('email').email('Invalid email format');
```

### URL
```javascript
validator.field('website').url('Invalid URL');
```

### Phone Number
```javascript
validator.field('phone').phone('Invalid phone number');
```

### Pattern Matching
```javascript
validator.field('zipCode')
    .pattern(/^\d{5}$/, 'Must be 5 digits');
```

### Alphanumeric
```javascript
validator.field('username').alphanumeric();
```

### Numeric Only
```javascript
validator.field('code').numeric();
```

### Letters Only
```javascript
validator.field('name').alpha();
```

### One Of
```javascript
validator.field('country')
    .oneOf(['US', 'CA', 'UK', 'AU'], 'Invalid country');
```

### Matches Other Field
```javascript
validator.field('confirmPassword')
    .matches('password', 'Passwords must match');
```

## Advanced Validators

### Strong Password
```javascript
const Validators = require('./validator').Validators;

validator.field('password')
    .required()
    .minLength(8)
    .custom(
        Validators.strongPassword,
        'Password must include uppercase, lowercase, number, and special character'
    );
```

### Credit Card (Luhn Algorithm)
```javascript
validator.field('cardNumber')
    .custom(Validators.creditCard, 'Invalid credit card number');
```

### Date Validation
```javascript
// Valid date
validator.field('birthdate')
    .custom(Validators.date, 'Invalid date');

// Future date
validator.field('appointmentDate')
    .custom(Validators.futureDate, 'Date must be in the future');

// Past date
validator.field('birthdate')
    .custom(Validators.pastDate, 'Birthdate must be in the past');

// Minimum age
validator.field('birthdate')
    .custom(Validators.age(18), 'Must be at least 18 years old');
```

### Username
```javascript
validator.field('username')
    .custom(Validators.username, 'Username must be 3-20 characters, letters/numbers/underscore/hyphen only');
```

### Zip Code
```javascript
validator.field('zipCode')
    .custom(Validators.zipCode('US'), 'Invalid US zip code');

validator.field('postalCode')
    .custom(Validators.zipCode('UK'), 'Invalid UK postal code');
```

## Custom Validators

### Synchronous Custom Validator
```javascript
validator.field('username')
    .custom(
        (value) => !value.includes('admin'),
        'Username cannot contain "admin"'
    );
```

### Asynchronous Custom Validator
```javascript
validator.field('username')
    .async(
        async (value) => {
            const response = await fetch(`/api/check-username/${value}`);
            const data = await response.json();
            return data.available;
        },
        'Username is already taken'
    );
```

## Conditional Validation

```javascript
validator.field('companyName')
    .when(
        (values) => values.accountType === 'business',
        (value) => value && value.length >= 3
    );

// companyName is only required when accountType is 'business'
```

## Cross-Field Validation

```javascript
validator.field('confirmPassword')
    .required()
    .matches('password', 'Passwords must match');

validator.field('endDate')
    .custom(
        (value, allValues) => {
            return new Date(value) > new Date(allValues.startDate);
        },
        'End date must be after start date'
    );
```

## Validation Results

```javascript
const result = await validator.validate(formData);

console.log(result);
// {
//     isValid: false,
//     fields: {
//         email: { isValid: true, errors: [] },
//         password: { isValid: false, errors: ['Password is too short'] }
//     },
//     errors: {
//         password: ['Password is too short']
//     }
// }
```

## Single Field Validation

```javascript
const result = await validator.validateField('email', 'user@example.com');

if (result.isValid) {
    console.log('Email is valid');
} else {
    console.log('Errors:', result.errors);
}
```

## Complete Example

```javascript
const validator = new FormValidator();

// User registration form
validator.field('username')
    .required('Username is required')
    .minLength(3, 'Username must be at least 3 characters')
    .maxLength(20, 'Username must be at most 20 characters')
    .alphanumeric('Username can only contain letters and numbers')
    .async(
        async (value) => {
            const response = await fetch(`/api/check-username/${value}`);
            return response.ok;
        },
        'Username is already taken'
    );

validator.field('email')
    .required('Email is required')
    .email('Invalid email format')
    .async(
        async (value) => {
            const response = await fetch(`/api/check-email/${value}`);
            return response.ok;
        },
        'Email is already registered'
    );

validator.field('password')
    .required('Password is required')
    .minLength(8, 'Password must be at least 8 characters')
    .custom(
        Validators.strongPassword,
        'Password must include uppercase, lowercase, number, and special character'
    );

validator.field('confirmPassword')
    .required('Please confirm your password')
    .matches('password', 'Passwords do not match');

validator.field('age')
    .required('Age is required')
    .numeric('Age must be a number')
    .min(18, 'You must be at least 18 years old')
    .max(120, 'Invalid age');

validator.field('website')
    .url('Invalid website URL');

validator.field('phone')
    .required('Phone number is required')
    .phone('Invalid phone number format');

validator.field('termsAccepted')
    .custom(
        (value) => value === true || value === 'true',
        'You must accept the terms and conditions'
    );

// Validate form
const formData = {
    username: 'john123',
    email: 'john@example.com',
    password: 'SecurePass123!',
    confirmPassword: 'SecurePass123!',
    age: '25',
    website: 'https://example.com',
    phone: '+1-555-123-4567',
    termsAccepted: true
};

const result = await validator.validate(formData);

if (result.isValid) {
    // Submit form
    await submitForm(formData);
} else {
    // Display errors
    displayErrors(result.errors);
}
```

## HTML Form Integration

```html
<!DOCTYPE html>
<html>
<head>
    <title>Form Validation Demo</title>
</head>
<body>
    <form id="registration-form">
        <div>
            <label>Username:</label>
            <input type="text" name="username" />
            <span class="error" id="username-error"></span>
        </div>

        <div>
            <label>Email:</label>
            <input type="email" name="email" />
            <span class="error" id="email-error"></span>
        </div>

        <div>
            <label>Password:</label>
            <input type="password" name="password" />
            <span class="error" id="password-error"></span>
        </div>

        <div>
            <label>Confirm Password:</label>
            <input type="password" name="confirmPassword" />
            <span class="error" id="confirmPassword-error"></span>
        </div>

        <button type="submit">Register</button>
    </form>

    <script src="validator.js"></script>
    <script>
        const validator = new FormValidator();

        validator.field('username')
            .required()
            .minLength(3)
            .alphanumeric();

        validator.field('email')
            .required()
            .email();

        validator.field('password')
            .required()
            .minLength(8);

        validator.field('confirmPassword')
            .required()
            .matches('password');

        document.getElementById('registration-form').addEventListener('submit', async (e) => {
            e.preventDefault();

            // Clear previous errors
            document.querySelectorAll('.error').forEach(el => el.textContent = '');

            // Get form data
            const formData = new FormData(e.target);
            const values = Object.fromEntries(formData);

            // Validate
            const result = await validator.validate(values);

            if (result.isValid) {
                alert('Form is valid!');
                // Submit form
            } else {
                // Display errors
                for (const [field, errors] of Object.entries(result.errors)) {
                    const errorEl = document.getElementById(`${field}-error`);
                    if (errorEl) {
                        errorEl.textContent = errors[0];
                    }
                }
            }
        });

        // Real-time validation
        document.querySelectorAll('input').forEach(input => {
            input.addEventListener('blur', async (e) => {
                const field = e.target.name;
                const value = e.target.value;

                const formData = new FormData(document.getElementById('registration-form'));
                const allValues = Object.fromEntries(formData);

                const result = await validator.validateField(field, value, allValues);

                const errorEl = document.getElementById(`${field}-error`);
                if (errorEl) {
                    errorEl.textContent = result.isValid ? '' : result.errors[0];
                }
            });
        });
    </script>
</body>
</html>
```

## React Integration

```jsx
import { useState } from 'react';
import { FormValidator } from './validator';

const validator = new FormValidator();

validator.field('email').required().email();
validator.field('password').required().minLength(8);

function LoginForm() {
    const [values, setValues] = useState({ email: '', password: '' });
    const [errors, setErrors] = useState({});

    const handleChange = (e) => {
        setValues({ ...values, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e) => {
        e.preventDefault();

        const result = await validator.validate(values);

        if (result.isValid) {
            // Submit form
            console.log('Valid!', values);
        } else {
            setErrors(result.errors);
        }
    };

    return (
        <form onSubmit={handleSubmit}>
            <div>
                <input
                    name="email"
                    value={values.email}
                    onChange={handleChange}
                />
                {errors.email && <span>{errors.email[0]}</span>}
            </div>

            <div>
                <input
                    name="password"
                    type="password"
                    value={values.password}
                    onChange={handleChange}
                />
                {errors.password && <span>{errors.password[0]}</span>}
            </div>

            <button type="submit">Login</button>
        </form>
    );
}
```

## Testing

```javascript
const { FormValidator } = require('./validator');

describe('FormValidator', () => {
    let validator;

    beforeEach(() => {
        validator = new FormValidator();
    });

    test('validates required field', async () => {
        validator.field('username').required();

        const result = await validator.validate({ username: '' });
        expect(result.isValid).toBe(false);
        expect(result.errors.username).toBeDefined();
    });

    test('validates email format', async () => {
        validator.field('email').email();

        const valid = await validator.validate({ email: 'user@example.com' });
        expect(valid.isValid).toBe(true);

        const invalid = await validator.validate({ email: 'invalid' });
        expect(invalid.isValid).toBe(false);
    });

    test('validates password match', async () => {
        validator.field('confirmPassword').matches('password');

        const match = await validator.validate({
            password: 'secret',
            confirmPassword: 'secret'
        });
        expect(match.isValid).toBe(true);

        const noMatch = await validator.validate({
            password: 'secret',
            confirmPassword: 'different'
        });
        expect(noMatch.isValid).toBe(false);
    });
});
```

## License

MIT License
