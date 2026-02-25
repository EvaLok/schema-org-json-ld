<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Event;
use EvaLok\SchemaOrgJsonLd\v1\Schema\EventAttendanceModeEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\Schema\EventStatusType;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Place;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VirtualLocation;
use PHPUnit\Framework\TestCase;

final class EventTest extends TestCase {
	public function testMinimalOutput(): void {
		$event = new Event(
			name: 'Jazz Night',
			startDate: '2026-04-01T20:00:00+02:00',
			location: new Place(
				name: 'Main Theater',
				address: new PostalAddress(streetAddress: '123 Main Street'),
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $event);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Event', $obj->{'@type'});
		$this->assertEquals('Jazz Night', $obj->name);
		$this->assertEquals('2026-04-01T20:00:00+02:00', $obj->startDate);
		$this->assertEquals('Place', $obj->location->{'@type'});
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$event = new Event(
			name: 'Jazz Night',
			startDate: '2026-04-01T20:00:00+02:00',
			location: new Place(
				name: 'Main Theater',
				address: new PostalAddress(streetAddress: '123 Main Street'),
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $event);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'endDate'));
		$this->assertFalse(property_exists($obj, 'eventAttendanceMode'));
		$this->assertFalse(property_exists($obj, 'eventStatus'));
		$this->assertFalse(property_exists($obj, 'image'));
		$this->assertFalse(property_exists($obj, 'offers'));
		$this->assertFalse(property_exists($obj, 'organizer'));
		$this->assertFalse(property_exists($obj, 'performer'));
		$this->assertFalse(property_exists($obj, 'previousStartDate'));
	}

	public function testFullOutputWithNestedTypesAndEnum(): void {
		$event = new Event(
			name: 'Jazz Night',
			startDate: '2026-04-01T20:00:00+02:00',
			location: new Place(
				name: 'Main Theater',
				address: new PostalAddress(
					streetAddress: '123 Main Street',
					addressLocality: 'Amsterdam',
					addressRegion: 'NH',
					postalCode: '1011AB',
					addressCountry: 'NL',
				),
			),
			description: 'An evening of live jazz music.',
			endDate: '2026-04-01T22:00:00+02:00',
			eventStatus: EventStatusType::EventRescheduled,
			image: ['https://example.com/event.jpg'],
			offers: [
				new Offer(
					url: 'https://example.com/tickets',
					priceCurrency: 'EUR',
					price: 29.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
			organizer: new Organization(name: 'Jazz Org'),
			performer: [new Person(name: 'John Coltrane')],
			previousStartDate: '2026-03-25T20:00:00+01:00',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $event);
		$obj = json_decode($json);

		$this->assertEquals('An evening of live jazz music.', $obj->description);
		$this->assertEquals('2026-04-01T22:00:00+02:00', $obj->endDate);
		$this->assertEquals('https://schema.org/EventRescheduled', $obj->eventStatus);
		$this->assertEquals('https://example.com/event.jpg', $obj->image[0]);
		$this->assertEquals('Offer', $obj->offers[0]->{'@type'});
		$this->assertEquals('Organization', $obj->organizer->{'@type'});
		$this->assertEquals('Person', $obj->performer[0]->{'@type'});
		$this->assertEquals('2026-03-25T20:00:00+01:00', $obj->previousStartDate);
	}

	public function testSingleOfferAndPerformerAreSupported(): void {
		$event = new Event(
			name: 'Jazz Night',
			startDate: '2026-04-01T20:00:00+02:00',
			location: new Place(
				name: 'Main Theater',
				address: new PostalAddress(streetAddress: '123 Main Street'),
			),
			offers: new Offer(
				url: 'https://example.com/tickets',
				priceCurrency: 'EUR',
				price: 29.99,
				itemCondition: OfferItemCondition::NewCondition,
				availability: ItemAvailability::InStock,
			),
			performer: new Person(name: 'John Coltrane'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $event);
		$obj = json_decode($json);

		$this->assertEquals('Offer', $obj->offers->{'@type'});
		$this->assertEquals('Person', $obj->performer->{'@type'});
	}

	public function testOnlineEventWithVirtualLocationAndAttendanceMode(): void {
		$event = new Event(
			name: 'Online Jazz Night',
			startDate: '2026-04-01T20:00:00+02:00',
			location: new VirtualLocation(url: 'https://example.com/join'),
			eventAttendanceMode: EventAttendanceModeEnumeration::OnlineEventAttendanceMode,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $event);
		$obj = json_decode($json);

		$this->assertEquals('VirtualLocation', $obj->location->{'@type'});
		$this->assertEquals('https://example.com/join', $obj->location->url);
		$this->assertEquals('https://schema.org/OnlineEventAttendanceMode', $obj->eventAttendanceMode);
	}

	public function testMixedEventWithPlaceAndVirtualLocationArray(): void {
		$event = new Event(
			name: 'Hybrid Jazz Night',
			startDate: '2026-04-01T20:00:00+02:00',
			location: [
				new Place(name: 'Main Theater'),
				new VirtualLocation(url: 'https://example.com/join'),
			],
			eventAttendanceMode: EventAttendanceModeEnumeration::MixedEventAttendanceMode,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $event);
		$obj = json_decode($json);

		$this->assertEquals('Place', $obj->location[0]->{'@type'});
		$this->assertEquals('VirtualLocation', $obj->location[1]->{'@type'});
		$this->assertEquals('https://schema.org/MixedEventAttendanceMode', $obj->eventAttendanceMode);
	}
}
