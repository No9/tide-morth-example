# tide-morth-example

An example application using mongodb rust tide and handlebars A.K.A MoRTH stack 

It's prime focus is to demonstrate a JSON centric approach to tide and to provde a project structure that should feel familiar to folks who have used JS Handlebars, pybars or Java Handlebars projects.

If you are evaluating tide patterns please look at [Rust Tide Example](https://github.com/jbr/tide-example).

Once this project has matured it's intended that it will enter the [Rust Tide Example Contributions](https://github.com/jbr/tide-example#contributing) section in some form.


## Comments

### JSON 

Currently a generic data access layer is provided by https://github.com/no9/tide-mongodb-dal 

**Advantages**

* Fast development for simple CRUD apps as no ORM is required.

* Accessible to developers familiar with Javascript constructs

**Disadvatages**

* No type enforcement of values means that validation could be more verbose.

### Types

T.B.D
