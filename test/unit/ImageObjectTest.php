<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ImageObject;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use PHPUnit\Framework\TestCase;

final class ImageObjectTest extends TestCase {
	public function testMinimalOutput(): void {
		$imageObject = new ImageObject(contentUrl: 'https://example.com/image.jpg');
		$json = JsonLdGenerator::SchemaToJson(schema: $imageObject);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ImageObject', $obj->{'@type'});
		$this->assertEquals('https://example.com/image.jpg', $obj->contentUrl);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$imageObject = new ImageObject(contentUrl: 'https://example.com/image.jpg');
		$json = JsonLdGenerator::SchemaToJson(schema: $imageObject);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'url'));
		$this->assertFalse(property_exists($obj, 'name'));
		$this->assertFalse(property_exists($obj, 'caption'));
		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'width'));
		$this->assertFalse(property_exists($obj, 'height'));
		$this->assertFalse(property_exists($obj, 'license'));
		$this->assertFalse(property_exists($obj, 'acquireLicensePage'));
		$this->assertFalse(property_exists($obj, 'creditText'));
		$this->assertFalse(property_exists($obj, 'copyrightNotice'));
		$this->assertFalse(property_exists($obj, 'creator'));
		$this->assertFalse(property_exists($obj, 'datePublished'));
		$this->assertFalse(property_exists($obj, 'uploadDate'));
	}

	public function testFullOutputWithCreator(): void {
		$imageObject = new ImageObject(
			contentUrl: 'https://example.com/image.jpg',
			url: 'https://example.com/image-page',
			name: 'Example Image',
			caption: 'A nice image.',
			description: 'Image description.',
			width: '800',
			height: '600 px',
			license: 'https://example.com/license',
			acquireLicensePage: 'https://example.com/acquire-license',
			creditText: 'Photo by Example Studio',
			copyrightNotice: 'Copyright 2026 Example',
			creator: new Organization(name: 'Example Studio'),
			datePublished: '2026-01-15',
			uploadDate: '2026-01-14',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $imageObject);
		$obj = json_decode($json);

		$this->assertEquals('https://example.com/image.jpg', $obj->contentUrl);
		$this->assertEquals('https://example.com/image-page', $obj->url);
		$this->assertEquals('Example Image', $obj->name);
		$this->assertEquals('A nice image.', $obj->caption);
		$this->assertEquals('Image description.', $obj->description);
		$this->assertEquals('800', $obj->width);
		$this->assertEquals('600 px', $obj->height);
		$this->assertEquals('https://example.com/license', $obj->license);
		$this->assertEquals('https://example.com/acquire-license', $obj->acquireLicensePage);
		$this->assertEquals('Photo by Example Studio', $obj->creditText);
		$this->assertEquals('Copyright 2026 Example', $obj->copyrightNotice);
		$this->assertEquals('Organization', $obj->creator->{'@type'});
		$this->assertEquals('Example Studio', $obj->creator->name);
		$this->assertEquals('2026-01-15', $obj->datePublished);
		$this->assertEquals('2026-01-14', $obj->uploadDate);
		$this->assertIsString($obj->width);
		$this->assertIsString($obj->height);
	}
}
