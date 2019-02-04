const addon = require('../native/index.node')

const { User } = addon

const x = new User(0, 'John', 'Doe', 'johndoe@gmail.com')

console.log(x)

module.exports = addon
