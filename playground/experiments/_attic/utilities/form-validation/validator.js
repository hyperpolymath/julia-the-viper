/**
 * Comprehensive Form Validation Library
 *
 * Features:
 * - Built-in validators (email, URL, phone, etc.)
 * - Custom validators
 * - Async validation
 * - Conditional validation
 * - Cross-field validation
 * - Custom error messages
 */

class ValidationRule {
    constructor(validator, message) {
        this.validator = validator;
        this.message = message;
    }

    async validate(value, allValues = {}) {
        const isValid = await this.validator(value, allValues);
        return {
            isValid,
            message: isValid ? null : this.message
        };
    }
}

class FieldValidator {
    constructor(fieldName) {
        this.fieldName = fieldName;
        this.rules = [];
        this.isRequired = false;
    }

    required(message = `${this.fieldName} is required`) {
        this.isRequired = true;
        this.rules.push(new ValidationRule(
            (value) => value !== undefined && value !== null && value !== '',
            message
        ));
        return this;
    }

    min(minValue, message = `${this.fieldName} must be at least ${minValue}`) {
        this.rules.push(new ValidationRule(
            (value) => !value || value >= minValue,
            message
        ));
        return this;
    }

    max(maxValue, message = `${this.fieldName} must be at most ${maxValue}`) {
        this.rules.push(new ValidationRule(
            (value) => !value || value <= maxValue,
            message
        ));
        return this;
    }

    minLength(minLength, message = `${this.fieldName} must be at least ${minLength} characters`) {
        this.rules.push(new ValidationRule(
            (value) => !value || value.length >= minLength,
            message
        ));
        return this;
    }

    maxLength(maxLength, message = `${this.fieldName} must be at most ${maxLength} characters`) {
        this.rules.push(new ValidationRule(
            (value) => !value || value.length <= maxLength,
            message
        ));
        return this;
    }

    email(message = `${this.fieldName} must be a valid email`) {
        this.rules.push(new ValidationRule(
            (value) => {
                if (!value) return true;
                const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
                return emailRegex.test(value);
            },
            message
        ));
        return this;
    }

    url(message = `${this.fieldName} must be a valid URL`) {
        this.rules.push(new ValidationRule(
            (value) => {
                if (!value) return true;
                try {
                    new URL(value);
                    return true;
                } catch {
                    return false;
                }
            },
            message
        ));
        return this;
    }

    phone(message = `${this.fieldName} must be a valid phone number`) {
        this.rules.push(new ValidationRule(
            (value) => {
                if (!value) return true;
                const phoneRegex = /^\+?[\d\s\-()]+$/;
                return phoneRegex.test(value) && value.replace(/\D/g, '').length >= 10;
            },
            message
        ));
        return this;
    }

    pattern(regex, message = `${this.fieldName} format is invalid`) {
        this.rules.push(new ValidationRule(
            (value) => !value || regex.test(value),
            message
        ));
        return this;
    }

    alphanumeric(message = `${this.fieldName} must contain only letters and numbers`) {
        return this.pattern(/^[a-zA-Z0-9]+$/, message);
    }

    numeric(message = `${this.fieldName} must contain only numbers`) {
        return this.pattern(/^\d+$/, message);
    }

    alpha(message = `${this.fieldName} must contain only letters`) {
        return this.pattern(/^[a-zA-Z]+$/, message);
    }

    oneOf(values, message = `${this.fieldName} must be one of: ${values.join(', ')}`) {
        this.rules.push(new ValidationRule(
            (value) => !value || values.includes(value),
            message
        ));
        return this;
    }

    matches(fieldName, message = `${this.fieldName} must match ${fieldName}`) {
        this.rules.push(new ValidationRule(
            (value, allValues) => !value || value === allValues[fieldName],
            message
        ));
        return this;
    }

    custom(validator, message = `${this.fieldName} is invalid`) {
        this.rules.push(new ValidationRule(validator, message));
        return this;
    }

    async(validator, message = `${this.fieldName} is invalid`) {
        this.rules.push(new ValidationRule(
            async (value) => await validator(value),
            message
        ));
        return this;
    }

    when(condition, thenValidator) {
        this.rules.push(new ValidationRule(
            async (value, allValues) => {
                if (condition(allValues)) {
                    return await thenValidator(value, allValues);
                }
                return true;
            },
            'Conditional validation failed'
        ));
        return this;
    }

    async validate(value, allValues = {}) {
        const errors = [];

        for (const rule of this.rules) {
            const result = await rule.validate(value, allValues);
            if (!result.isValid) {
                errors.push(result.message);
            }
        }

        return {
            isValid: errors.length === 0,
            errors
        };
    }
}

class FormValidator {
    constructor() {
        this.fields = {};
    }

    field(fieldName) {
        if (!this.fields[fieldName]) {
            this.fields[fieldName] = new FieldValidator(fieldName);
        }
        return this.fields[fieldName];
    }

    async validate(values) {
        const results = {};
        let isValid = true;

        for (const [fieldName, validator] of Object.entries(this.fields)) {
            const fieldValue = values[fieldName];
            const result = await validator.validate(fieldValue, values);

            results[fieldName] = result;

            if (!result.isValid) {
                isValid = false;
            }
        }

        return {
            isValid,
            fields: results,
            errors: Object.fromEntries(
                Object.entries(results)
                    .filter(([_, result]) => !result.isValid)
                    .map(([field, result]) => [field, result.errors])
            )
        };
    }

    async validateField(fieldName, value, allValues = {}) {
        const validator = this.fields[fieldName];
        if (!validator) {
            throw new Error(`No validator found for field: ${fieldName}`);
        }

        return await validator.validate(value, allValues);
    }
}

// Built-in validators
const Validators = {
    required: (message) => (value) => {
        return value !== undefined && value !== null && value !== '';
    },

    email: (value) => {
        if (!value) return true;
        return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value);
    },

    url: (value) => {
        if (!value) return true;
        try {
            new URL(value);
            return true;
        } catch {
            return false;
        }
    },

    minLength: (min) => (value) => {
        return !value || value.length >= min;
    },

    maxLength: (max) => (value) => {
        return !value || value.length <= max;
    },

    min: (min) => (value) => {
        return !value || Number(value) >= min;
    },

    max: (max) => (value) => {
        return !value || Number(value) <= max;
    },

    pattern: (regex) => (value) => {
        return !value || regex.test(value);
    },

    oneOf: (values) => (value) => {
        return !value || values.includes(value);
    },

    matches: (otherField) => (value, allValues) => {
        return !value || value === allValues[otherField];
    },

    creditCard: (value) => {
        if (!value) return true;
        // Luhn algorithm
        const digits = value.replace(/\D/g, '');
        if (digits.length < 13 || digits.length > 19) return false;

        let sum = 0;
        let isEven = false;

        for (let i = digits.length - 1; i >= 0; i--) {
            let digit = parseInt(digits[i]);

            if (isEven) {
                digit *= 2;
                if (digit > 9) digit -= 9;
            }

            sum += digit;
            isEven = !isEven;
        }

        return sum % 10 === 0;
    },

    date: (value) => {
        if (!value) return true;
        const date = new Date(value);
        return !isNaN(date.getTime());
    },

    futureDate: (value) => {
        if (!value) return true;
        const date = new Date(value);
        return date > new Date();
    },

    pastDate: (value) => {
        if (!value) return true;
        const date = new Date(value);
        return date < new Date();
    },

    age: (minAge) => (value) => {
        if (!value) return true;
        const birthDate = new Date(value);
        const age = (new Date() - birthDate) / (365.25 * 24 * 60 * 60 * 1000);
        return age >= minAge;
    },

    strongPassword: (value) => {
        if (!value) return true;
        return (
            value.length >= 8 &&
            /[a-z]/.test(value) &&
            /[A-Z]/.test(value) &&
            /\d/.test(value) &&
            /[!@#$%^&*]/.test(value)
        );
    },

    username: (value) => {
        if (!value) return true;
        return /^[a-zA-Z0-9_-]{3,20}$/.test(value);
    },

    zipCode: (country = 'US') => (value) => {
        if (!value) return true;
        const patterns = {
            US: /^\d{5}(-\d{4})?$/,
            UK: /^[A-Z]{1,2}\d{1,2}[A-Z]?\s?\d[A-Z]{2}$/i,
            CA: /^[A-Z]\d[A-Z]\s?\d[A-Z]\d$/i
        };
        return patterns[country]?.test(value) || false;
    }
};

// Example usage
function demo() {
    console.log('='.repeat(60));
    console.log('Form Validation Library Demo');
    console.log('='.repeat(60));

    // Create form validator
    const validator = new FormValidator();

    // Define validation rules
    validator.field('username')
        .required()
        .minLength(3)
        .maxLength(20)
        .alphanumeric('Username must contain only letters and numbers');

    validator.field('email')
        .required()
        .email();

    validator.field('password')
        .required()
        .minLength(8)
        .custom(Validators.strongPassword, 'Password must be strong (uppercase, lowercase, number, special character)');

    validator.field('confirmPassword')
        .required()
        .matches('password', 'Passwords must match');

    validator.field('age')
        .required()
        .numeric()
        .min(18, 'You must be at least 18 years old');

    validator.field('website')
        .url();

    validator.field('phone')
        .phone();

    // Test data
    const validData = {
        username: 'john123',
        email: 'john@example.com',
        password: 'SecurePass123!',
        confirmPassword: 'SecurePass123!',
        age: '25',
        website: 'https://example.com',
        phone: '+1-555-123-4567'
    };

    const invalidData = {
        username: 'jo',
        email: 'invalid-email',
        password: 'weak',
        confirmPassword: 'different',
        age: '15',
        website: 'not-a-url',
        phone: '123'
    };

    // Validate
    console.log('\n--- Validating Valid Data ---');
    validator.validate(validData).then(result => {
        console.log('Is Valid:', result.isValid);
        if (!result.isValid) {
            console.log('Errors:', JSON.stringify(result.errors, null, 2));
        } else {
            console.log('All fields are valid!');
        }
    });

    console.log('\n--- Validating Invalid Data ---');
    validator.validate(invalidData).then(result => {
        console.log('Is Valid:', result.isValid);
        console.log('Errors:', JSON.stringify(result.errors, null, 2));
    });
}

// Export for Node.js
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { FormValidator, FieldValidator, Validators };
}

// Run demo if executed directly
if (typeof require !== 'undefined' && require.main === module) {
    demo();
}
