class Solution:
    def findAllRecipes(self, recipes: List[str], ingredients: List[List[str]], supplies: List[str]) -> List[str]:
        supplies = {s: True for s in supplies}
        
        counts = {recipe: 1 for recipe in recipes}

        canMake = defaultdict(list)

        q = deque([])
        ans = []
        for (i, recipe) in enumerate(recipes):
            counts[recipe] = len(ingredients[i])
            for j, ingredient in enumerate(ingredients[i]):
                if ingredient in supplies:
                    counts[recipe] -= 1
                else:
                    canMake[ingredient].append(recipe)
            if counts[recipe] == 0:
                q.append(recipe)
        
        while q:
            ingredient = q.popleft()
            ans.append(ingredient)
            for recipe in canMake[ingredient]:
                counts[recipe] -= 1
                if counts[recipe] == 0:
                    q.append(recipe)
        
        return ans

