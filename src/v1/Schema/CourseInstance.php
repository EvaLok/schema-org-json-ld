<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class CourseInstance extends TypedSchema {
	public const A_SCHEMA_TYPE = 'CourseInstance';

	public function __construct(
		public null|string $courseMode = null,
		public null|Person $instructor = null,
		public null|Schedule $courseSchedule = null,
		public null|string $courseWorkload = null,
	) {}
}
