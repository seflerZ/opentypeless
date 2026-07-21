import { useAppStore } from '../../stores/appStore'

/** Simple level-meter bar that grows left-to-right with voice volume. */
export function Waveform() {
  const volume = useAppStore((s) => s.audioVolume)

  // Clamp and convert to percentage for the bar width
  const pct = Math.round(Math.min(100, Math.max(0, volume * 100)))

  return (
    <div className="flex-1 h-[5px] bg-white/10 rounded-full overflow-hidden mx-1">
      <div
        className="h-full bg-white/70 rounded-full transition-all duration-75 ease-out"
        style={{ width: `${pct}%` }}
      />
    </div>
  )
}
