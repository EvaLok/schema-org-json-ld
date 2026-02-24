<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\HowToStep;
use EvaLok\SchemaOrgJsonLd\v1\Schema\NutritionInformation;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Recipe;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use PHPUnit\Framework\TestCase;

final class RecipeTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Recipe(
			name: 'Grandma Cookies',
			image: ['https://example.com/cookies.jpg'],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Recipe', $obj->{'@type'});
		$this->assertEquals('Grandma Cookies', $obj->name);
		$this->assertEquals('https://example.com/cookies.jpg', $obj->image[0]);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Recipe(
			name: 'Grandma Cookies',
			image: ['https://example.com/cookies.jpg'],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'author'));
		$this->assertFalse(property_exists($obj, 'nutrition'));
		$this->assertFalse(property_exists($obj, 'recipeIngredient'));
		$this->assertFalse(property_exists($obj, 'recipeInstructions'));
		$this->assertFalse(property_exists($obj, 'aggregateRating'));
		$this->assertFalse(property_exists($obj, 'review'));
	}

	public function testFullOutputWithNestedTypes(): void {
		$schema = new Recipe(
			name: 'Grandma Cookies',
			image: ['https://example.com/cookies.jpg'],
			author: new Person(name: 'Jane Doe'),
			datePublished: '2026-01-20',
			description: 'Classic soft cookies.',
			prepTime: 'PT15M',
			cookTime: 'PT20M',
			totalTime: 'PT35M',
			keywords: 'cookies,dessert,classic',
			recipeYield: '24 cookies',
			recipeCategory: 'Dessert',
			recipeCuisine: 'American',
			recipeIngredient: ['2 cups flour', '1 cup sugar'],
			recipeInstructions: [
				new HowToStep(text: 'Preheat oven to 180C.'),
				new HowToStep(text: 'Bake until golden.'),
			],
			nutrition: new NutritionInformation(calories: '240 calories'),
			aggregateRating: new AggregateRating(ratingValue: 4.8, reviewCount: 12),
			review: [
				new Review(
					author: 'Alice',
					reviewRating: new Rating(ratingValue: 5),
					reviewBody: 'Amazing cookies!',
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Person', $obj->author->{'@type'});
		$this->assertEquals('HowToStep', $obj->recipeInstructions[0]->{'@type'});
		$this->assertEquals('2 cups flour', $obj->recipeIngredient[0]);
		$this->assertEquals('NutritionInformation', $obj->nutrition->{'@type'});
		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertEquals('Review', $obj->review[0]->{'@type'});
	}

	public function testAuthorSupportsOrganizationAndReviewSupportsSingleObject(): void {
		$schema = new Recipe(
			name: 'Grandma Cookies',
			image: ['https://example.com/cookies.jpg'],
			author: new Organization(name: 'Cookie Studio'),
			review: new Review(
				author: 'Alice',
				reviewRating: new Rating(ratingValue: 5),
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Organization', $obj->author->{'@type'});
		$this->assertEquals('Review', $obj->review->{'@type'});
	}
}
