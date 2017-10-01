(function () {
    angular.module("shoppinglist", []);


    angular.module("shoppinglist")
        .directive("recipes", recipes);

    function recipes() {
        return {
            restrict: 'E',
            scope: {},
            templateUrl: 'recipes.html',
            controllerAs: 'vm',
            controller: RecipesController
        };
    }



    RecipesController.$inject = ['$http']
    function RecipesController($http) {
        var vm = this;
        vm.recipes = [{
            name: 'Recipe 1'
        }, {
            name: 'Recipe 2'
        }];
        vm.selectedRecipe = null;
        vm.selectRecipe = selectRecipe;
        vm.newIngredient = "";
        vm.selectedIngredient = null;
        vm.ingredients = [];
        vm.ingredientFilter = "";
        vm.ingredientFilterFn = ingredientFilterFn;
        vm.addIngredient = addIngredient;
        vm.selectIngredient = selectIngredient;
        vm.addToRecipe = addToRecipe;
        vm.newAmount = 0;
        $http.get('../get-recipes')
            .then(function (request) {
                vm.recipes = request.data;
            });
        $http.get('../get-ingredients')
            .then(function (request) {
                vm.ingredients = request.data;
            });
        vm.newRecipe = {};
        vm.add = add;

        function add() {
            $http.get('../add-recipe?name=' + vm.newRecipe.name);
            vm.recipes.push(vm.newRecipe);
            vm.newRecipe = {};
        }

        function selectRecipe(recipe) {
            vm.selectedRecipe = recipe;
        }

        function addToRecipe() {
            var ingredientKey = vm.selectedIngredient.key;
            var amount = vm.newAmount;
            $http.get('../add-ingredient-to-recipe?ingredient_key=' + ingredientKey
                + "&amount=" + amount
                + "&recipe_key=" + vm.selectedRecipe.key).then(function (response) {
                    vm.selectedRecipe.ingredients.push({
                        amount: amount,
                        ingredient: vm.selectedIngredient
                    });
                });

            vm.newIngredient = "";
            vm.newAmount = 0;
        }

        function ingredientFilterFn() {
            return _.filter(vm.ingredients, function (ingredient) {
                return ingredient.name.search(vm.ingredientFilter) >= 0;
            });
        }

        function addIngredient() {
            $http.get('../add-ingredient?name=' + vm.ingredientFilter).then(function (response) {
                vm.ingredients.push({
                    name: vm.ingredientFilter,
                    ingredient_key: response.data
                });
            });
        }

        function selectIngredient(ingredientKey) {
            vm.selectedIngredient = ingredientKey;
        }
    }



    angular.module('shoppinglist').directive('shoppinglist', shoppinglist);

    function shoppinglist() {
        return {
            restrict: 'E',
            scope: {},
            templateUrl: 'shoppinglist.html',
            controllerAs: 'vm',
            controller: ShoppinglistController
        }
    }

    ShoppinglistController.$inject = ['$http'];
    function ShoppinglistController($http) {
        var vm = this;
        vm.list = [];
        vm.newIngredient = "";
        vm.selectedIngredient = null;
        vm.ingredients = [];
        vm.ingredientFilter = "";
        vm.newAmount = 0;
        vm.add = add;
        vm.ingredientFilterFn = ingredientFilterFn;
        vm.addIngredient = addIngredient;
        vm.selectIngredient = selectIngredient;
        $http.get('../get-shoppinglist')
            .then(function (request) {
                vm.list = request.data;
            });
        $http.get('../get-ingredients')
            .then(function (request) {
                vm.ingredients = request.data;
            });

        function add() {
            var ingredientKey = vm.selectedIngredient.key;
            var amount = vm.newAmount;
            $http.get('../add-shoppinglist?ingredient_key=' + ingredientKey
                + "&amount=" + amount).then(function (response) {
                    vm.list.push({
                        key: response.data,
                        ingredient_key: ingredientKey,
                        amount: amount,
                        recipe_key: null
                    });
                });

            vm.newIngredient = "";
            vm.newAmount = 0;
        }

        function ingredientFilterFn() {
            return _.filter(vm.ingredients, function (ingredient) {
                return ingredient.name.search(vm.ingredientFilter) >= 0;
            });
        }

        function addIngredient() {
            $http.get('../add-ingredient?name=' + vm.ingredientFilter).then(function (response) {
                vm.ingredients.push({
                    name: vm.ingredientFilter,
                    ingredient_key: response.data
                });
            });
        }

        function selectIngredient(ingredientKey) {
            vm.selectedIngredient = ingredientKey;
        }
    }

})();