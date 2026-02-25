/**
 * Miou — Mascotte chat kawaii de Miyukini.
 * Rendu en SVG inline pour le centre de l'inventaire.
 */
interface MiouMascotProps {
  size?: number;
  className?: string;
}

export function MiouMascot({ size = 180, className }: MiouMascotProps) {
  return (
    <svg
      width={size}
      height={size * 1.3}
      viewBox="0 0 180 234"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      {/* Ears */}
      <path d="M45 75 L30 20 L70 55 Z" fill="#ffd4e5" stroke="#ffb6d1" strokeWidth="2" />
      <path d="M135 75 L150 20 L110 55 Z" fill="#ffd4e5" stroke="#ffb6d1" strokeWidth="2" />
      {/* Inner ears */}
      <path d="M48 68 L38 30 L65 56 Z" fill="#ffb6d1" opacity="0.5" />
      <path d="M132 68 L142 30 L115 56 Z" fill="#ffb6d1" opacity="0.5" />

      {/* Head */}
      <ellipse cx="90" cy="100" rx="60" ry="55" fill="#ffd4e5" />
      <ellipse cx="90" cy="100" rx="60" ry="55" fill="url(#headGradient)" />

      {/* Cheek blush */}
      <ellipse cx="50" cy="115" rx="14" ry="8" fill="#ff9ec4" opacity="0.4" />
      <ellipse cx="130" cy="115" rx="14" ry="8" fill="#ff9ec4" opacity="0.4" />

      {/* Eyes — closed happy */}
      <path d="M62 98 Q72 88 82 98" stroke="#5a3d4a" strokeWidth="2.5" strokeLinecap="round" fill="none" />
      <path d="M98 98 Q108 88 118 98" stroke="#5a3d4a" strokeWidth="2.5" strokeLinecap="round" fill="none" />

      {/* Nose */}
      <ellipse cx="90" cy="108" rx="3" ry="2" fill="#e8829e" />

      {/* Mouth — happy */}
      <path d="M82 114 Q90 122 98 114" stroke="#5a3d4a" strokeWidth="1.8" strokeLinecap="round" fill="none" />

      {/* Whiskers */}
      <line x1="25" y1="105" x2="55" y2="108" stroke="#d4a0b5" strokeWidth="1" opacity="0.6" />
      <line x1="25" y1="115" x2="55" y2="114" stroke="#d4a0b5" strokeWidth="1" opacity="0.6" />
      <line x1="155" y1="105" x2="125" y2="108" stroke="#d4a0b5" strokeWidth="1" opacity="0.6" />
      <line x1="155" y1="115" x2="125" y2="114" stroke="#d4a0b5" strokeWidth="1" opacity="0.6" />

      {/* Body */}
      <ellipse cx="90" cy="180" rx="45" ry="40" fill="#ffd4e5" />
      <ellipse cx="90" cy="180" rx="45" ry="40" fill="url(#bodyGradient)" />

      {/* Belly */}
      <ellipse cx="90" cy="185" rx="25" ry="22" fill="#fff0f5" opacity="0.7" />

      {/* Paws */}
      <ellipse cx="60" cy="210" rx="14" ry="8" fill="#ffc0d8" />
      <ellipse cx="120" cy="210" rx="14" ry="8" fill="#ffc0d8" />

      {/* Tail */}
      <path d="M135 180 Q165 160 155 130 Q150 115 160 105" stroke="#ffc0d8" strokeWidth="8" strokeLinecap="round" fill="none" />

      {/* Star on chest — Miyukini emblem */}
      <polygon
        points="90,165 93,173 102,173 95,179 97,187 90,182 83,187 85,179 78,173 87,173"
        fill="#fff"
        opacity="0.8"
      />

      {/* Sparkles */}
      <g opacity="0.6">
        <text x="25" y="70" fontSize="10" fill="#ffd4e5">✦</text>
        <text x="150" y="65" fontSize="8" fill="#ffd4e5">✧</text>
        <text x="15" y="140" fontSize="7" fill="#ffb6d1">✦</text>
        <text x="160" y="150" fontSize="9" fill="#ffb6d1">✧</text>
      </g>

      <defs>
        <radialGradient id="headGradient" cx="50%" cy="40%" r="60%">
          <stop offset="0%" stopColor="#ffe8f0" stopOpacity="0.3" />
          <stop offset="100%" stopColor="#ffd4e5" stopOpacity="0" />
        </radialGradient>
        <radialGradient id="bodyGradient" cx="50%" cy="30%" r="60%">
          <stop offset="0%" stopColor="#ffe8f0" stopOpacity="0.3" />
          <stop offset="100%" stopColor="#ffd4e5" stopOpacity="0" />
        </radialGradient>
      </defs>
    </svg>
  );
}
