# tide-morth-example

An example application using mongodb rust tide and handlebars A.K.A MoRTH stack 

It's prime focus is to provde a project structure that should feel familiar to folks who have used JS Handlebars, pybars or Java Handlebars projects.

If you are evaluating tide patterns please look at [Rust Tide Example](https://github.com/jbr/tide-example).

Once this project has matured it's intended that it will enter the [Rust Tide Example Contributions](https://github.com/jbr/tide-example#contributing) section in some form.

This example requires a local mongo instance which can be ran with:

```
docker run -d -p 27017:27017 -v ~/data:/data/db mongo:4.2
```

## Comments

* Fast development for simple CRUD apps as no data access layer is defined as a decorated struct - See models/cities.rs

* Accessible to developers familiar with Javascript constructs
