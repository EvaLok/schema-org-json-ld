<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Movie;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use PHPUnit\Framework\TestCase;

final class MovieTest extends TestCase {
	public function testMinimalOutput(): void {
		$movie = new Movie(
			name: 'Interstellar',
			image: 'https://example.com/interstellar.jpg',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $movie);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Movie', $obj->{'@type'});
		$this->assertEquals('Interstellar', $obj->name);
		$this->assertEquals('https://example.com/interstellar.jpg', $obj->image);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$movie = new Movie(
			name: 'Interstellar',
			image: 'https://example.com/interstellar.jpg',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $movie);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'aggregateRating'));
		$this->assertFalse(property_exists($obj, 'dateCreated'));
		$this->assertFalse(property_exists($obj, 'director'));
		$this->assertFalse(property_exists($obj, 'review'));
		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'actor'));
	}

	public function testFullOutputWithNestedTypes(): void {
		$movie = new Movie(
			name: 'Interstellar',
			image: 'https://example.com/interstellar.jpg',
			aggregateRating: new AggregateRating(
				ratingValue: 4.7,
				ratingCount: 1540,
			),
			dateCreated: '2014-11-07',
			director: new Person(name: 'Christopher Nolan'),
			review: new Review(
				author: 'Jane Critic',
				reviewRating: new Rating(ratingValue: 5),
				reviewBody: 'A visually stunning movie.',
			),
			description: 'A team travels through a wormhole in search of a new home for humanity.',
			actor: [
				new Person(name: 'Matthew McConaughey'),
				new Person(name: 'Anne Hathaway'),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $movie);
		$obj = json_decode($json);

		$this->assertEquals('Interstellar', $obj->name);
		$this->assertEquals('https://example.com/interstellar.jpg', $obj->image);
		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertEquals(4.7, $obj->aggregateRating->ratingValue);
		$this->assertEquals(1540, $obj->aggregateRating->ratingCount);
		$this->assertEquals('2014-11-07', $obj->dateCreated);
		$this->assertEquals('Person', $obj->director->{'@type'});
		$this->assertEquals('Christopher Nolan', $obj->director->name);
		$this->assertEquals('Review', $obj->review->{'@type'});
		$this->assertEquals('Jane Critic', $obj->review->author);
		$this->assertEquals('Rating', $obj->review->reviewRating->{'@type'});
		$this->assertEquals(5, $obj->review->reviewRating->ratingValue);
		$this->assertEquals('A visually stunning movie.', $obj->review->reviewBody);
		$this->assertEquals('A team travels through a wormhole in search of a new home for humanity.', $obj->description);
		$this->assertEquals('Person', $obj->actor[0]->{'@type'});
		$this->assertEquals('Matthew McConaughey', $obj->actor[0]->name);
		$this->assertEquals('Person', $obj->actor[1]->{'@type'});
		$this->assertEquals('Anne Hathaway', $obj->actor[1]->name);
	}
}
