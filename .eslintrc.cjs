module.exports = {
  extends: [
    '@antfu',
  ],
  rules: {
    'no-prototype-builtins': 'off',
    'prefer-const': 'warn',
    'no-console': 'off',
    'no-debugger': 'off',
    'no-unused-vars': 'warn',
    'no-case-declarations': 'off',
    'vue/no-unused-components': 'warn',
    'vue/no-unused-vars': 'warn',
    'vue/no-multi-spaces': 'error',
    'vue/html-self-closing': 'off',
    'vue/no-v-html': 'off',
    'vue/mustache-interpolation-spacing': 'error',
    'vue/max-attributes-per-line': ['warn', {
      singleline: {
        max: 1,
      },
      multiline: {
        max: 1,
      },
    }],
    'vue/first-attribute-linebreak': ['warn', {
      singleline: 'beside',
      multiline: 'below',
    }],
    'vue/html-closing-bracket-spacing': ['error', {
      startTag: 'never',
      endTag: 'never',
      selfClosingTag: 'always',
    }],
    'no-trailing-spaces': 'warn',
    '@typescript-eslint/no-explicit-any': 'warn',
    '@typescript-eslint/no-this-alias': 'warn',
    '@typescript-eslint/no-empty-function': 'warn',
    '@typescript-eslint/no-non-null-assertion': 'off',
    '@typescript-eslint/no-inferrable-types': 'off',
    '@typescript-eslint/no-unused-vars': 'warn',
    '@typescript-eslint/no-use-before-define': 'off',
    '@typescript-eslint/ban-ts-ignore': 'off',
    '@typescript-eslint/member-delimiter-style': ['error', {
      multiline: {
        delimiter: 'none',
        requireLast: true,
      },
      singleline: {
        delimiter: 'semi',
        requireLast: false,
      },
    }],
    '@typescript-eslint/brace-style': ['warn', '1tbs', {
      allowSingleLine: false,
    }],
    'semi': ['error', 'never'],
    'array-bracket-spacing': ['warn', 'never'],
    'object-curly-spacing': ['warn', 'always'],
    'arrow-parens': ['warn', 'always'],
    'arrow-spacing': 'warn',
    'use-isnan': 'error',
    'no-irregular-whitespace': 'error',
    'no-iterator': 'error',
    'no-label-var': 'error',
    'indent': ['error', 2, {
      SwitchCase: 1,
    }],
    'quotes': ['error', 'single', { avoidEscape: true, allowTemplateLiterals: true }],
    'keyword-spacing': 'error',
    'no-undef': 'error',
    'no-underscore-dangle': 'off',
    'camelcase': ['error', { properties: 'never' }],
    'no-multi-spaces': 'warn',
    'no-multiple-empty-lines': ['warn', { max: 2 }],
    'space-before-function-paren': ['off', 'always'],
    'block-spacing': ['error', 'always'],
    'brace-style': ['error', '1tbs', { allowSingleLine: true }],
    'comma-spacing': ['error', { before: false, after: true }],
    'curly': ['error', 'multi-line'],
    'dot-location': ['error', 'property'],
    'eol-last': 'error',
    'func-call-spacing': ['error', 'never'],
    'key-spacing': ['error', { beforeColon: false, afterColon: true }],
    'no-mixed-spaces-and-tabs': 'error',
    'no-whitespace-before-property': 'error',
    'semi-spacing': ['error', { before: false, after: true }],
    'space-before-blocks': ['error', 'always'],
    'space-in-parens': ['error', 'never'],
    'space-infix-ops': 'error',
    'strict': 2,
    'new-parens': 'error',
    'no-class-assign': 'error',
    'no-constant-condition': ['warn', { checkLoops: false }],
    'no-delete-var': 'error',
    'no-dupe-args': 'error',
    'no-dupe-class-members': 'error',
    'no-dupe-keys': 'error',
    'no-duplicate-case': 'error',
    'no-empty': 'warn',
    'no-empty-character-class': 'error',
    'no-empty-pattern': 'error',
    'no-ex-assign': 'error',
    'no-fallthrough': 'error',
    'no-global-assign': 'error',
    'no-invalid-regexp': 'error',
    'no-self-assign': 'error',
    'no-shadow-restricted-names': 'error',
    'no-template-curly-in-string': 'error',
    'no-unreachable': 'warn',
    'no-control-regex': 'error',
    'no-use-before-define': 'off',
    'no-redeclare': 'error',
    'no-useless-escape': 'warn',
    'no-useless-rename': 'error',
    'one-var': ['error', { initialized: 'never' }],
    'new-cap': ['error', { newIsCap: true, capIsNew: false }],
    'unicorn/prefer-includes': 'warn',
  },
}
