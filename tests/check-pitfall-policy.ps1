$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot

function Read-RepoFile {
    param([string]$Path)
    $fullPath = Join-Path $repoRoot $Path
    if (-not (Test-Path -LiteralPath $fullPath)) {
        throw "Missing required file: $Path"
    }
    Get-Content -LiteralPath $fullPath -Raw
}

function Assert-Contains {
    param(
        [string]$Content,
        [string]$Needle,
        [string]$Label
    )
    $normalizedContent = ($Content -split '\s+') -join ' '
    $normalizedNeedle = ($Needle -split '\s+') -join ' '
    if (-not $normalizedContent.Contains($normalizedNeedle)) {
        throw "$Label missing expected text: $Needle"
    }
}

$pitfalls = Read-RepoFile ".pitfall/gauge-pitfalls.md"
$invariants = Read-RepoFile ".pitfall/gauge-invariants.md"
$readme = Read-RepoFile "README.md"
$plan = Read-RepoFile "PRODUCT_PLAN.md"
$claude = Read-RepoFile "CLAUDE.md"
$roles = Read-RepoFile ".roles/ROLE.md"
$adoption = Read-RepoFile "docs/adoption/README.md"
$finding = Read-RepoFile "docs/findings/2026-06-frequency-span-of-service.md"
$review = Read-RepoFile "docs/vtrace/REVIEW.md"
$boundaries = Read-RepoFile "docs/pitfall-boundaries.v1.json"

Assert-Contains $pitfalls "## GAUGE-PF-05: First Rail Finding Becomes Public Authority" "pitfall section"
Assert-Contains $pitfalls "**Status:** MITIGATED" "pitfall status"
Assert-Contains $pitfalls "tests/check-pitfall-policy.ps1" "pitfall retained test"
Assert-Contains $invariants "Public Reuse Requires Boundary Review" "public boundary invariant"
Assert-Contains $invariants "**Status:** MITIGATED" "public boundary invariant status"
Assert-Contains $invariants "DIM-07-only scope" "public boundary invariant enforcement"

Assert-Contains $readme "Research lab only" "README research boundary"
Assert-Contains $readme "not an engineering study, timetable, or FRA/Amtrak endorsement" "README research boundary"
Assert-Contains $readme "not an engineering study" "README authority boundary"
Assert-Contains $readme "environmental review" "README authority boundary"
Assert-Contains $readme "procurement plan" "README authority boundary"
Assert-Contains $readme "timetable" "README authority boundary"
Assert-Contains $readme "advocacy brief" "README authority boundary"
Assert-Contains $readme "Direct GAUGE reuse requires a named downstream consumer, a bounded versioned surface, and consumer-owned compatibility tests" "README provider boundary"
Assert-Contains $readme "That is evidence about the tested corpus, not a national construction mandate" "README finding boundary"
Assert-Contains $plan "Broader corpus coverage, additional dimensions, and proposal review remain future work" "product plan future work boundary"
Assert-Contains $plan "No engineering alignment, environmental review, procurement, or timetable" "product plan non-goals"
Assert-Contains $claude "No ridership or trip-time claims dressed as solved engineering" "CLAUDE engineering boundary"
Assert-Contains $claude "Do not start implementation code until the relevant work package is accepted" "CLAUDE implementation gate"

Assert-Contains $roles "First public rail finding" "role public finding PITFALL gate"
Assert-Contains $roles "Rail Planner (FRA/national)" "role planner gate"
Assert-Contains $roles "Freight-Host Railroad Realist" "role host railroad gate"
Assert-Contains $roles "Citation Auditor" "role citation gate"
Assert-Contains $roles "Scope Keeper" "role scope gate"
Assert-Contains $roles "Numeracy Checker" "role numeracy gate"

Assert-Contains $adoption "Public use does not create an engineering study, environmental review, procurement plan, timetable, advocacy brief" "adoption authority boundary"
Assert-Contains $finding "Dimension assessed: DIM-07 Frequency / Span of Service (only)" "finding DIM-07-only scope"
Assert-Contains $finding "Frequency is the only scored dimension" "finding single-dimension note"
Assert-Contains $finding "12 of 13 dimensions are reported as explicit" "finding unassessed dimensions"
Assert-Contains $finding "EmptyRegion" "finding unassessed dimensions"
Assert-Contains $review "Role Review Matrix" "review role matrix"
Assert-Contains $review "PITFALL Boundary" "review PITFALL boundary"
Assert-Contains $review "No finding currently has public-authority status" "review public authority boundary"

Assert-Contains $boundaries '"schema": "pitfall-boundaries.v1"' "boundary schema"
Assert-Contains $boundaries "first-rail-finding-is-not-public-authority" "boundary rule"
Assert-Contains $boundaries "research-lab evidence only" "boundary research posture"
Assert-Contains $boundaries "They do not authorize an engineering study, timetable, procurement plan, advocacy mandate, FRA/Amtrak/state-DOT/host-railroad position, funding instruction, or national construction mandate" "boundary held claims"
Assert-Contains $boundaries "named release artifact, visible unassessed dimensions, cited corpus/source basis, reproducible run, and full role review" "boundary promotion prerequisites"

Write-Output "GAUGE PITFALL policy checks passed"
