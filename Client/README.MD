# get expo
* https://expo.io/learn

# Run
* expo start --ios

# Lint
* yarn Lint

# Tree of the project
```
src
├── actions
├── app.js
├── components
├── reducers
│   ├── index.js
│   └── user
│       └── index.js
├── utlis
│   └── index.js
└── views
    ├── Conversation
    │   └── index.js
    └── Home
        └── index.js
```

* View or made for view, they must to be as simple as possible and only call components, the view should keep the state and the logique of any of they children.
* Components or use by view but need to be as modulaire as possible the if  logique is need it should be in the parent.
* Action will dispatch all call to any  api.
* reducer will dispatch all ansewer of any api.
* utlis just some utils function.

* Middlware is messing for WS, very imporant for perf when workink whit redux and WS.

* Today the code in views is not good, it's need to be move to components and refacto to be more modulable.
