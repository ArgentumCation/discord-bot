fn main() {
    println!("Hello, world!");
}

struct Roles {
    pronouns: Vec<Pronoun>,
    degree: Option<Degree>,
    college: Option<College>,
}

enum Pronoun {
    He,
    She,
    They,
    Neo,
}

enum Degree {
    MdDvm,
    Masters,
    Phd,
    PostDoc,
    Alumni,
    Undergrad,
}

enum College {
    UniversityCollege,
    GraduateShool,
    Veterinary,
    Textiles,
    Sciences,
    Management(Management),
    NaturalResources,
    Chass(Chass),
    Engineering(Engineering),
    Education(Education),
    Design(Design),
    Agriculture(Agriculture),
}

enum Engineering {
    Operations,
    Manufacturing,
    GoldenLeaf,
    Nuclear,
    MechAero,
    MatSci,
    Ise,
    Ece,
    Csc,
    Civil,
    Chem,
    Bme,
}

enum Agriculture {
    AgriculturalAndResourceEconomics,
    PlantAndMicrobialBiology,
    EntomologyAndPlantPathology,
    HorticulturalScience,
    CropAndSoilSciences,
    AgriculturalAndHumanSciences,
    MolecularAndStructuralBiochemistry,
    FoodBioprocessingAndNutritionSciences,
    BiologicalAndAgriculturalEngineering,
    PrestageDepartmentOfPoultryScience,
    AppliedEcology,
    AnimalScience,
}

enum Design {
    Architecture,
    ArtDesign,
    DesignStudies,
    GraphicDesign,
    IndustrialDesign,
    LandscapeArchitectureAndEnvironmentalPlanning,
}

enum Education {
    EducationalLeadershipPolicyandHumanDevelopment,
    ScienceTechnologyEngineeringandMathematicsEducation,
    TeacherEducationandLearningSciences,
}

enum Chass {
    Communication,
    English,
    ForeignLanguagesandLiteratures,
    History,
    InterdisciplinaryStudies,
    PhilosophyandReligion,
    Psychology,
    PublicandInternationalAffairs,
    SocialWork,
    SociologyandAnthropology,
}

enum Management {
    Accounting,
    BusinessManagement,
    Economics,
    Finance,
    ManagementInnovationAndEntrepreneurship,
    SKEMA,
}

enum Sciences {
    BiologicalSciences,
    Chemistry,
    MarineEarthandAtmosphericSciences,
    Mathematics,
    Physics,
    Statistics,
}

enum Textiles {
    TextileEngineeringChemistryandScience,
    TextileAndApparelTechnologyandManagement,
    ZeisExtension,
}

enum Veterinary {
    ClinicalSciences,
    MolecularBiomedicalSciences,
    PopulationHealthAndPathobiology,
}

enum GraduateShool {
    AdvancedAnalytics,
}

enum UniversityCollege {
    AirForceROTC,
    ArmyROTC,
    NavalROTC,
    ExploratoryStudies,
    HealthAndExerciseStudies,
    Music,
    Theatre,
}
