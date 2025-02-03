use crate::base::pass_registry;
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

/// A Binaryen optimization pass.
///
/// These have the same names as given on the command line to
/// `wasm-opt`, but with Rust capitalization conventions.
// Keep these in the same order as PassRegistry::registerPasses
#[non_exhaustive]
#[derive(Clone, Debug, EnumIter, EnumString, IntoStaticStr, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Pass {
    /// Lower unaligned loads and stores to smaller aligned ones.
    AlignmentLowering,
    /// Async/await style transform, allowing pausing and resuming.
    Asyncify,
    /// Tries to avoid reinterpret operations via more loads.
    AvoidReinterprets,
    /// Removes arguments to calls in an lto-like manner.
    Dae,
    /// Removes arguments to calls in an lto-like manner, and optimizes where removed.
    DaeOptimizing,
    /// Refine and merge abstract (never-created) types.
    AbstractTypeRefining,
    /// Reduce # of locals by coalescing.
    CoalesceLocals,
    /// Reduce # of locals by coalescing and learning.
    CoalesceLocalsLearning,
    /// Push code forward, potentially making it not always execute.
    CodePushing,
    /// Fold code, merging duplicates.
    CodeFolding,
    /// Hoist repeated constants to a local.
    ConstHoisting,
    /// Propagate constant struct field values.
    Cfp,
    /// Removes unreachable code.
    Dce,
    /// Forces all loads and stores to have alignment 1.
    Dealign,
    /// Instrument the wasm to convert NaNs into 0 at runtime.
    DeNan,
    /// Turns indirect calls into direct ones.
    Directize,
    /// Discards global effect info.
    DiscardGlobalEffects,
    /// Optimizes using the DataFlow SSA IR.
    Dfo,
    /// Dump DWARF debug info sections from the read binary.
    DwarfDump,
    /// Removes duplicate imports.
    DuplicateImportElimination,
    /// Removes duplicate functions.
    DuplicateFunctionElimination,
    /// Emit the target features section in the output.
    EmitTargetFeatures,
    /// Leaves just one function (useful for debugging).
    ExtractFunction,
    /// Leaves just one function selected by index.
    ExtractFunctionIndex,
    /// Flattens out code, removing nesting.
    Flatten,
    /// Emulates function pointer casts, allowing incorrect indirect calls to (sometimes) work.
    FpCastEmu,
    /// Reports function metrics.
    FuncMetrics,
    /// Generate dynCall fuctions used by emscripten ABI.
    GenerateDyncalls,
    /// Generate dynCall functions used by emscripten ABI, but only for functions with i64 in their signature (which cannot be invoked via the wasm table without JavaScript BigInt support).
    GenerateI64Dyncalls,
    /// Generate global effect info (helps later passes).
    GenerateGlobalEffects,
    /// Generate Stack IR.
    GenerateStackIr,
    /// Refine the types of globals.
    GlobalRefining,
    /// Globally optimize GC types.
    Gto,
    /// Globally optimize struct values.
    Gsi,
    /// Grand unified flow analyses.
    ///
    /// Optimize the entire program using information about what content can actually appear in each location.
    Gufa,
    /// GUFA plus add casts for all inferences.
    GufaCastAll,
    /// Gufa plus local optimizations in functions we modified.
    GufaOptimizing,
    /// Apply more specific subtypes to type fields where possible.
    TypeRefining,
    /// Replace GC allocations with locals.
    Heap2Local,
    /// Inline __original_main into main.
    InlineMain,
    /// Inline functions (you probably want inlining-optimizing).
    Inlining,
    /// Inline functions and optimizes where we inlined.
    InliningOptimizing,
    /// Lower away binaryen intrinsics.
    IntrinsicLowering,
    /// Wrap imports and exports for JavaScript promise integration.
    Jspi,
    /// Legalizes i64 types on the import/export boundary.
    LegalizeJsInterface,
    /// Legalizes i64 types on the import/export boundary in a minimal manner, only on things only JS will call.
    LegalizeJsInterfaceMinimally,
    /// Common subexpression elimination inside basic blocks.
    LocalCse,
    /// Apply more specific subtypes to locals where possible.
    LocalSubtyping,
    /// Instrument the build with logging of where execution goes.
    LogExecution,
    /// Lower all uses of i64s to use i32s instead.
    I64ToI32Lowering,
    /// Instrument the build with code to intercept all loads and stores.
    InstrumentLocals,
    /// Instrument the build with code to intercept all loads and stores.
    InstrumentMemory,
    /// Loop invariant code motion.
    Licm,
    /// Attempt to merge segments to fit within web limits.
    LimitSegments,
    /// Lower loads and stores to a 64-bit memory to instead use a 32-bit one.
    Memory64Lowering,
    /// Packs memory into separate segments, skipping zeros.
    MemoryPacking,
    /// Merges blocks to their parents.
    MergeBlocks,
    /// Merges similar functions when benefical.
    MergeSimilarFunctions,
    /// Merges locals when beneficial.
    MergeLocals,
    /// Reports metrics.
    Metrics,
    /// Minifies import names (only those, and not export names), and emits a mapping to the minified ones.
    MinifyImports,
    /// Minifies both import and export names, and emits a mapping to the minified ones.
    MinifyImportsAndExports,
    /// Minifies both import and export names, and emits a mapping to the minified ones, and minifies the modules as well.
    MinifyImportsAndExportsAndModules,
    /// Apply the assumption that asyncify imports always unwind, and we never rewind.
    ModAsyncifyAlwaysAndOnlyUnwind,
    /// Apply the assumption that asyncify never unwinds.
    ModAsyncifyNeverUnwind,
    /// Creates specialized versions of functions.
    Monomorphize,
    /// Creates specialized versions of functions (even if unhelpful).
    MonomorphizeAlways,
    /// Combines multiple memories into a single memory.
    MultiMemoryLowering,
    /// Combines multiple memories into a single memory, trapping if the read or write is larger than the length of the memory's data.
    MultiMemoryLoweringWithBoundsChecks,
    /// Name list.
    Nm,
    /// (Re)name all heap types.
    NameTypes,
    /// Reduces calls to code that only runs once.
    OnceReduction,
    /// Optimizes added constants into load/store offsets.
    OptimizeAddedConstants,
    /// Optimizes added constants into load/store offsets, propagating them across locals too.
    OptimizeAddedConstantsPropagate,
    /// Eliminate and reuse casts.
    OptimizeCasts,
    /// Optimizes instruction combinations.
    OptimizeInstructions,
    /// Optimize Stack IR.
    OptimizeStackIr,
    /// Pick load signs based on their uses.
    PickLoadSigns,
    /// Tranform Binaryen IR into Poppy IR.
    Poppify,
    /// Miscellaneous optimizations for Emscripten-generated code.
    PostEmscripten,
    /// Early optimize of the instruction combinations for js.
    OptimizeForJs,
    /// Computes compile-time evaluatable expressions.
    Precompute,
    /// Computes compile-time evaluatable expressions and propagates.
    PrecomputePropagate,
    /// Print in s-expression format.
    Print,
    /// Print in minified s-expression format.
    PrintMinified,
    /// Print options for enabled features.
    PrintFeatures,
    /// Print in full s-expression format.
    PrintFull,
    /// Print call graph.
    PrintCallGraph,
    /// Print a map of function indexes to names.
    PrintFunctionMap,
    /// (Alias for print-function-map).
    Symbolmap,
    /// Print out Stack IR (useful for internal debugging).
    PrintStackIr,
    /// Removes operations incompatible with js.
    RemoveNonJsOps,
    /// Removes imports and replaces them with nops.
    RemoveImports,
    /// Removes memory segments.
    RemoveMemory,
    /// Removes breaks from locations that are not needed.
    RemoveUnusedBrs,
    /// Removes unused module elements.
    RemoveUnusedModuleElements,
    /// Removes unused module elements that are not functions.
    RemoveUnusedNonfunctionModuleElements,
    /// Removes names from locations that are never branched to.
    RemoveUnusedNames,
    /// Remove unused private GC types.
    RemoveUnusedTypes,
    /// Sorts functions by name (useful for debugging).
    ReorderFunctionsByName,
    /// Sorts functions by access frequency.
    ReorderFunctions,
    /// Sorts globals by access frequency.
    ReorderGlobals,
    /// Sorts locals by access frequency.
    RecorderLocals,
    /// Re-optimize control flow using the relooper algorithm.
    Rereloop,
    /// Remove redundant local.sets.
    Rse,
    /// Write the module to binary, then read it.
    Roundtrip,
    /// Instrument loads and stores to check for invalid behavior.
    SafeHeap,
    /// Sets specified globals to specified values.
    SetGlobals,
    /// Remove params from function signature types where possible.
    SignaturePruning,
    /// Apply more specific subtypes to signature types where possible.
    SignatureRefining,
    /// Lower sign-ext operations to wasm mvp.
    SignextLowering,
    /// Miscellaneous globals-related optimizations.
    SimplifyGlobals,
    /// Miscellaneous globals-related optimizations, and optimizes where we replaced global.gets with constants.
    SimplifyGlobalsOptimizing,
    /// Miscellaneous locals-related optimizations.
    SimplifyLocals,
    /// Miscellaneous locals-related optimizations (no nesting at all; preserves flatness).
    SimplifyLocalsNonesting,
    /// Miscellaneous locals-related optimizations (no tees).
    SimplifyLocalsNotee,
    /// Miscellaneous locals-related optimizations (no structure).
    SimplifyLocalsNostructure,
    /// Miscellaneous locals-related optimizations (no tees or structure).
    SimplifyLocalsNoteeNostructure,
    /// Emit Souper IR in text form.
    Souperify,
    /// Emit Souper IR in text form (single-use nodes only).
    SouperifySingleUse,
    /// Spill pointers to the C stack (useful for Boehm-style GC).
    SpillPointers,
    /// Stub out unsupported JS operations.
    StubUnsupportedJs,
    /// Ssa-ify variables so that they have a single assignment.
    Ssa,
    /// Ssa-ify variables so that they have a single assignment, ignoring merges.
    SsaNomerge,
    /// Deprecated; same as strip-debug.
    Strip,
    /// Enforce limits on llvm's __stack_pointer global.
    StackCheck,
    /// Strip debug info (including the names section).
    StripDebug,
    /// Strip dwarf debug info.
    StripDwarf,
    /// Strip the wasm producers section.
    StripProducers,
    /// Strip EH instructions.
    StripEh,
    /// Strip the wasm target features section.
    StripTargetFeatuers,
    /// Replace trapping operations with clamping semantics.
    TrapModeClamp,
    /// Replace trapping operations with js semantics.
    TrapModeJs,
    /// Merge types to their supertypes where possible.
    TypeMerging,
    /// Create new nominal types to help other optimizations.
    TypeSsa,
    /// Removes local.tees, replacing them with sets and gets.
    Untee,
    /// Removes obviously unneeded code.
    Vacuum,
}

impl Pass {
    /// Get Binaryen's description of the pass.
    pub fn description(&self) -> String {
        // NB: This will abort if the name is invalid
        pass_registry::get_pass_description(self.into())
    }
}
