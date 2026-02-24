<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Answer extends TypedSchema {

	const A_SCHEMA_TYPE = 'Answer';

	public function __construct(
		public string $text,
	) {

	}
}
