<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum DayOfWeek: string {
	case Monday = 'https://schema.org/Monday';
	case Tuesday = 'https://schema.org/Tuesday';
	case Wednesday = 'https://schema.org/Wednesday';
	case Thursday = 'https://schema.org/Thursday';
	case Friday = 'https://schema.org/Friday';
	case Saturday = 'https://schema.org/Saturday';
	case Sunday = 'https://schema.org/Sunday';
}
