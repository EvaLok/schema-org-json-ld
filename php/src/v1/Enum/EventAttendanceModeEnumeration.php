<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Enum;

enum EventAttendanceModeEnumeration: string {
	case OfflineEventAttendanceMode = 'https://schema.org/OfflineEventAttendanceMode';
	case OnlineEventAttendanceMode = 'https://schema.org/OnlineEventAttendanceMode';
	case MixedEventAttendanceMode = 'https://schema.org/MixedEventAttendanceMode';
}
