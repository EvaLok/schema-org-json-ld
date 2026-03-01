<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class VideoObject extends TypedSchema {
	public const A_SCHEMA_TYPE = 'VideoObject';

	public function __construct(
		public string $name,
		/** @var string[] $thumbnailUrl */
		public array $thumbnailUrl,
		public string $uploadDate,
		public null|string $description = null,
		public null|string $contentUrl = null,
		public null|string $embedUrl = null,
		public null|string $duration = null,
		public null|string $expires = null,
		public null|string $regionsAllowed = null,
		/** @var InteractionCounter|InteractionCounter[]|null $interactionStatistic */
		public null|InteractionCounter|array $interactionStatistic = null,
		/** @var null|Clip[] $hasPart */
		public null|array $hasPart = null,
		public null|string $ineligibleRegion = null,
		public null|BroadcastEvent $publication = null,
	) {}
}
