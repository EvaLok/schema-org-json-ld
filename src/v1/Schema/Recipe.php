<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Recipe extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Recipe';

	public function __construct(
		public string $name,
		/** @var string[] $image */
		public array $image,
		public null|Person|Organization $author = null,
		public null|string $datePublished = null,
		public null|string $description = null,
		public null|string $prepTime = null,
		public null|string $cookTime = null,
		public null|string $totalTime = null,
		public null|string $keywords = null,
		public null|string $recipeYield = null,
		public null|string $recipeCategory = null,
		public null|string $recipeCuisine = null,
		/** @var string[] $recipeIngredient */
		public null|array $recipeIngredient = null,
		/** @var HowToStep[] $recipeInstructions */
		public null|array $recipeInstructions = null,
		public null|NutritionInformation $nutrition = null,
		public null|AggregateRating $aggregateRating = null,
		/** @var Review[] $review */
		public null|Review|array $review = null,
		public null|VideoObject $video = null,
	) {}
}
