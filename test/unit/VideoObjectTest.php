<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Clip;
use EvaLok\SchemaOrgJsonLd\v1\Schema\InteractionCounter;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VideoObject;
use PHPUnit\Framework\TestCase;

final class VideoObjectTest extends TestCase {
	public function testMinimalOutput(): void {
		$videoObject = new VideoObject(
			name: 'How to tie a tie',
			thumbnailUrl: ['https://example.com/thumb.jpg'],
			uploadDate: '2026-02-24',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $videoObject);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('VideoObject', $obj->{'@type'});
		$this->assertEquals('How to tie a tie', $obj->name);
		$this->assertEquals(['https://example.com/thumb.jpg'], $obj->thumbnailUrl);
		$this->assertEquals('2026-02-24', $obj->uploadDate);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$videoObject = new VideoObject(
			name: 'How to tie a tie',
			thumbnailUrl: ['https://example.com/thumb.jpg'],
			uploadDate: '2026-02-24',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $videoObject);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'description'));
		$this->assertFalse(property_exists($obj, 'contentUrl'));
		$this->assertFalse(property_exists($obj, 'embedUrl'));
		$this->assertFalse(property_exists($obj, 'duration'));
		$this->assertFalse(property_exists($obj, 'expires'));
		$this->assertFalse(property_exists($obj, 'regionsAllowed'));
		$this->assertFalse(property_exists($obj, 'interactionStatistic'));
		$this->assertFalse(property_exists($obj, 'hasPart'));
		$this->assertFalse(property_exists($obj, 'ineligibleRegion'));
	}

	public function testFullOutput(): void {
		$videoObject = new VideoObject(
			name: 'How to tie a tie',
			thumbnailUrl: ['https://example.com/thumb-1.jpg', 'https://example.com/thumb-2.jpg'],
			uploadDate: '2026-02-24',
			description: 'A short tutorial.',
			contentUrl: 'https://example.com/videos/tie.mp4',
			embedUrl: 'https://example.com/player/tie',
			duration: 'PT2M30S',
			expires: '2026-12-31T23:59:59+00:00',
			regionsAllowed: 'US,CA',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $videoObject);
		$obj = json_decode($json);

		$this->assertEquals('How to tie a tie', $obj->name);
		$this->assertEquals(['https://example.com/thumb-1.jpg', 'https://example.com/thumb-2.jpg'], $obj->thumbnailUrl);
		$this->assertEquals('2026-02-24', $obj->uploadDate);
		$this->assertEquals('A short tutorial.', $obj->description);
		$this->assertEquals('https://example.com/videos/tie.mp4', $obj->contentUrl);
		$this->assertEquals('https://example.com/player/tie', $obj->embedUrl);
		$this->assertEquals('PT2M30S', $obj->duration);
		$this->assertEquals('2026-12-31T23:59:59+00:00', $obj->expires);
		$this->assertEquals('US,CA', $obj->regionsAllowed);
	}

	public function testWithInteractionStatistic(): void {
		$videoObject = new VideoObject(
			name: 'How to tie a tie',
			thumbnailUrl: ['https://example.com/thumb.jpg'],
			uploadDate: '2026-02-24',
			interactionStatistic: new InteractionCounter(
				interactionType: 'https://schema.org/WatchAction',
				userInteractionCount: 54321,
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $videoObject);
		$obj = json_decode($json);

		$this->assertEquals('InteractionCounter', $obj->interactionStatistic->{'@type'});
		$this->assertEquals('https://schema.org/WatchAction', $obj->interactionStatistic->interactionType);
		$this->assertEquals(54321, $obj->interactionStatistic->userInteractionCount);
	}

	public function testWithIneligibleRegion(): void {
		$videoObject = new VideoObject(
			name: 'How to tie a tie',
			thumbnailUrl: ['https://example.com/thumb.jpg'],
			uploadDate: '2026-02-24',
			ineligibleRegion: 'US',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $videoObject);
		$obj = json_decode($json);

		$this->assertEquals('US', $obj->ineligibleRegion);
	}

	public function testWithHasPartKeyMoments(): void {
		$videoObject = new VideoObject(
			name: 'How to tie a tie',
			thumbnailUrl: ['https://example.com/thumb.jpg'],
			uploadDate: '2026-02-24',
			hasPart: [
				new Clip(
					name: 'Create the knot',
					startOffset: 30,
					url: 'https://example.com/video?t=30',
					endOffset: 75,
				),
				new Clip(
					name: 'Tighten and finish',
					startOffset: 76,
					url: 'https://example.com/video?t=76',
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $videoObject);
		$obj = json_decode($json);

		$this->assertCount(2, $obj->hasPart);
		$this->assertEquals('Clip', $obj->hasPart[0]->{'@type'});
		$this->assertEquals('Create the knot', $obj->hasPart[0]->name);
		$this->assertEquals(30, $obj->hasPart[0]->startOffset);
		$this->assertEquals('https://example.com/video?t=30', $obj->hasPart[0]->url);
		$this->assertEquals(75, $obj->hasPart[0]->endOffset);
		$this->assertObjectNotHasProperty('endOffset', $obj->hasPart[1]);
	}
}
