<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class SeekToAction extends TypedSchema {
	public const A_SCHEMA_TYPE = 'SeekToAction';
	public const PROPERTY_MAP = [
		'startOffsetInput' => 'startOffset-input',
	];

	public function __construct(
		public string $target,
		public string $startOffsetInput,
	) {}
}
