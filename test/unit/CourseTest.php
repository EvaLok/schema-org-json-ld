<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Course;
use EvaLok\SchemaOrgJsonLd\v1\Schema\CourseInstance;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Schedule;
use PHPUnit\Framework\TestCase;

final class CourseTest extends TestCase {
	public function testMinimalOutput(): void {
		$course = new Course(
			name: 'Introduction to Computer Science',
			description: 'Learn core computing concepts.',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $course);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Course', $obj->{'@type'});
		$this->assertEquals('Introduction to Computer Science', $obj->name);
		$this->assertEquals('Learn core computing concepts.', $obj->description);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$course = new Course(
			name: 'Introduction to Computer Science',
			description: 'Learn core computing concepts.',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $course);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'provider'));
		$this->assertFalse(property_exists($obj, 'offers'));
		$this->assertFalse(property_exists($obj, 'hasCourseInstance'));
		$this->assertFalse(property_exists($obj, 'courseCode'));
		$this->assertFalse(property_exists($obj, 'inLanguage'));
		$this->assertFalse(property_exists($obj, 'totalHistoricalEnrollment'));
		$this->assertFalse(property_exists($obj, 'aggregateRating'));
		$this->assertFalse(property_exists($obj, 'image'));
	}

	public function testFullOutputWithNestedSchemas(): void {
		$course = new Course(
			name: 'Introduction to Computer Science',
			description: 'Learn core computing concepts.',
			provider: new Organization(name: 'Example University'),
			offers: [
				new Offer(
					url: 'https://example.edu/courses/cs101',
					priceCurrency: 'USD',
					price: 199.00,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
			hasCourseInstance: [
				new CourseInstance(
					courseMode: 'online',
					instructor: new Person(name: 'Dr. Ada Lovelace'),
					courseSchedule: new Schedule(
						repeatFrequency: 'P1W',
						repeatCount: 10,
						startDate: '2026-09-01',
						endDate: '2026-11-10',
					),
					courseWorkload: 'PT22H',
				),
			],
			courseCode: 'CS101',
			inLanguage: 'en',
			totalHistoricalEnrollment: 1500,
			aggregateRating: new AggregateRating(
				ratingValue: 4.7,
				ratingCount: 325,
			),
			image: 'https://example.edu/images/cs101.jpg',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $course);
		$obj = json_decode($json);

		$this->assertEquals('Organization', $obj->provider->{'@type'});
		$this->assertEquals('Offer', $obj->offers[0]->{'@type'});
		$this->assertEquals('CourseInstance', $obj->hasCourseInstance[0]->{'@type'});
		$this->assertEquals('online', $obj->hasCourseInstance[0]->courseMode);
		$this->assertEquals('Person', $obj->hasCourseInstance[0]->instructor->{'@type'});
		$this->assertEquals('Schedule', $obj->hasCourseInstance[0]->courseSchedule->{'@type'});
		$this->assertEquals('P1W', $obj->hasCourseInstance[0]->courseSchedule->repeatFrequency);
		$this->assertEquals(10, $obj->hasCourseInstance[0]->courseSchedule->repeatCount);
		$this->assertEquals('PT22H', $obj->hasCourseInstance[0]->courseWorkload);
		$this->assertEquals('CS101', $obj->courseCode);
		$this->assertEquals('en', $obj->inLanguage);
		$this->assertEquals(1500, $obj->totalHistoricalEnrollment);
		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertEquals(4.7, $obj->aggregateRating->ratingValue);
		$this->assertEquals('https://example.edu/images/cs101.jpg', $obj->image);
	}

	public function testCourseInstanceCourseModeOmittedWhenNull(): void {
		$course = new Course(
			name: 'Introduction to Computer Science',
			description: 'Learn core computing concepts.',
			hasCourseInstance: [
				new CourseInstance(),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $course);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj->hasCourseInstance[0], 'courseMode'));
	}
}
