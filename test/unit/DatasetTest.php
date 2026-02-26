<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DataCatalog;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DataDownload;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Dataset;
use EvaLok\SchemaOrgJsonLd\v1\Schema\GeoShape;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Place;
use PHPUnit\Framework\TestCase;

final class DatasetTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Dataset(
			name: 'NCDC Storm Events Database',
			description: 'Storm Data is provided by the National Weather Service.',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Dataset', $obj->{'@type'});
		$this->assertEquals('NCDC Storm Events Database', $obj->name);
		$this->assertEquals('Storm Data is provided by the National Weather Service.', $obj->description);
	}

	public function testNullFieldsOmitted(): void {
		$schema = new Dataset(
			name: 'Test Dataset',
			description: 'A test dataset.',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertObjectNotHasProperty('url', $obj);
		$this->assertObjectNotHasProperty('sameAs', $obj);
		$this->assertObjectNotHasProperty('creator', $obj);
		$this->assertObjectNotHasProperty('funder', $obj);
		$this->assertObjectNotHasProperty('license', $obj);
		$this->assertObjectNotHasProperty('keywords', $obj);
		$this->assertObjectNotHasProperty('identifier', $obj);
		$this->assertObjectNotHasProperty('isAccessibleForFree', $obj);
		$this->assertObjectNotHasProperty('temporalCoverage', $obj);
		$this->assertObjectNotHasProperty('spatialCoverage', $obj);
		$this->assertObjectNotHasProperty('includedInDataCatalog', $obj);
		$this->assertObjectNotHasProperty('distribution', $obj);
		$this->assertObjectNotHasProperty('variableMeasured', $obj);
		$this->assertObjectNotHasProperty('measurementTechnique', $obj);
		$this->assertObjectNotHasProperty('version', $obj);
		$this->assertObjectNotHasProperty('alternateName', $obj);
		$this->assertObjectNotHasProperty('citation', $obj);
	}

	public function testWithCreatorOrganization(): void {
		$schema = new Dataset(
			name: 'Storm Events',
			description: 'Storm data.',
			creator: new Organization(name: 'NOAA'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Organization', $obj->creator->{'@type'});
		$this->assertEquals('NOAA', $obj->creator->name);
	}

	public function testWithDistributionArray(): void {
		$schema = new Dataset(
			name: 'Storm Events',
			description: 'Storm data.',
			distribution: [
				new DataDownload(contentUrl: 'https://example.com/data.csv', encodingFormat: 'text/csv'),
				new DataDownload(contentUrl: 'https://example.com/data.json', encodingFormat: 'application/json'),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertIsArray($obj->distribution);
		$this->assertCount(2, $obj->distribution);
		$this->assertEquals('DataDownload', $obj->distribution[0]->{'@type'});
		$this->assertEquals('https://example.com/data.csv', $obj->distribution[0]->contentUrl);
		$this->assertEquals('text/csv', $obj->distribution[0]->encodingFormat);
	}

	public function testWithSpatialCoverage(): void {
		$schema = new Dataset(
			name: 'Storm Events',
			description: 'Storm data.',
			spatialCoverage: new Place(
				name: 'New Zealand',
				geo: new GeoShape(box: '-47.0 166.0 -34.0 178.6'),
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Place', $obj->spatialCoverage->{'@type'});
		$this->assertEquals('New Zealand', $obj->spatialCoverage->name);
		$this->assertEquals('GeoShape', $obj->spatialCoverage->geo->{'@type'});
		$this->assertEquals('-47.0 166.0 -34.0 178.6', $obj->spatialCoverage->geo->box);
	}

	public function testWithKeywordsAndIdentifiers(): void {
		$schema = new Dataset(
			name: 'Storm Events',
			description: 'Storm data.',
			keywords: ['storm', 'weather', 'NOAA'],
			identifier: ['https://doi.org/10.1234/example'],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertIsArray($obj->keywords);
		$this->assertEquals(['storm', 'weather', 'NOAA'], $obj->keywords);
		$this->assertIsArray($obj->identifier);
		$this->assertEquals(['https://doi.org/10.1234/example'], $obj->identifier);
	}

	public function testFullOutput(): void {
		$schema = new Dataset(
			name: 'NCDC Storm Events Database',
			description: 'Storm Data is provided by the National Weather Service.',
			url: 'https://catalog.data.gov/dataset/ncdc-storm-events-database',
			sameAs: 'https://gis.ncdc.noaa.gov/geoportal/catalog/search/resource/details.page?id=gov.noaa.ncdc:C00510',
			creator: new Organization(name: 'National Weather Service'),
			funder: new Organization(name: 'NOAA'),
			license: 'https://creativecommons.org/licenses/by/4.0/',
			keywords: ['storm', 'weather', 'NOAA'],
			identifier: ['https://doi.org/10.1234/example'],
			isAccessibleForFree: true,
			temporalCoverage: '1950-01-01/2013-12-18',
			spatialCoverage: new Place(name: 'United States'),
			includedInDataCatalog: new DataCatalog(name: 'data.gov'),
			distribution: [
				new DataDownload(contentUrl: 'https://example.com/data.csv', encodingFormat: 'text/csv'),
			],
			variableMeasured: 'Storm events',
			measurementTechnique: 'Collected via storm reports',
			version: '1.0',
			alternateName: 'Storm Events',
			citation: 'NOAA Storm Events Database, Version 1.0',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Dataset', $obj->{'@type'});
		$this->assertEquals('NCDC Storm Events Database', $obj->name);
		$this->assertEquals('https://catalog.data.gov/dataset/ncdc-storm-events-database', $obj->url);
		$this->assertEquals('Organization', $obj->creator->{'@type'});
		$this->assertEquals('Organization', $obj->funder->{'@type'});
		$this->assertEquals('https://creativecommons.org/licenses/by/4.0/', $obj->license);
		$this->assertTrue($obj->isAccessibleForFree);
		$this->assertEquals('1950-01-01/2013-12-18', $obj->temporalCoverage);
		$this->assertEquals('Place', $obj->spatialCoverage->{'@type'});
		$this->assertEquals('DataCatalog', $obj->includedInDataCatalog->{'@type'});
		$this->assertEquals('1.0', $obj->version);
		$this->assertEquals('Storm Events', $obj->alternateName);
		$this->assertEquals('NOAA Storm Events Database, Version 1.0', $obj->citation);
	}
}
