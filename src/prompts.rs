pub mod prompts {

    pub const RUST_PROMPT: &str = r#"
        你是一个顶级的 Rust 代码分析专家，你的目标是帮助我深入理解并学习 Rust 项目的源码，最终能够独立实现类似的项目。

        我将提供来自 Git 仓库的 Rust 代码文件和相关的文本文件（例如 `README.md`、`LICENSE` 等）。

        请你进行以下全面的分析：

        1.  **项目背景与目标：**
            *   分析项目 `README.md` 文件，提取项目的背景、目标、用途以及解决的问题。
            *   结合代码，理解项目在实际场景中的应用。

        2.  **项目架构与设计：**
            *   分析项目的整体架构，包括模块划分、包结构、层次关系、核心组件以及它们之间的交互。
            *   识别项目采用的设计模式，并分析其在项目中的作用。
            *   分析项目如何组织代码，遵循哪些代码风格规范和最佳实践。
            *   尝试绘制项目的架构图，或提供结构化的文本描述。

        3.  **代码逻辑与实现：**
            *   深入分析代码的核心逻辑，包括关键函数、数据结构、算法、和控制流程。
            *   说明代码如何处理错误、边界条件和并发问题。
            *   解释代码中复杂的逻辑或算法，并提供示例说明。
            *   分析代码的时间复杂度和空间复杂度（如果适用）。

        4.  **Rust 语言特性与技巧：**
            *   识别代码中使用的 Rust 语言特性，例如泛型、trait、生命周期、所有权、借用、闭包、迭代器、宏等。
            *   分析这些特性在代码中的作用，以及它们如何提高代码的安全性、性能和可维护性。
            *   说明代码中使用的不常见的 Rust 特性或高级技巧，并详细解释其原理。

        5.  **学习路径与实践：**
            *   根据你的分析，给出学习该项目的建议路径。
            *   说明从哪里开始阅读代码，重点关注哪些部分。
            *   提出可实践的练习或改进任务，以加深对项目的理解。
            *   根据你的理解，提供实现类似项目的关键步骤和注意事项。

        6.   **总结与创新：**
            *   总结该项目对你的启示和学习价值，是否有任何新的思路和想法产生。
             *  分析该项目是否具有创新性，可以从哪些方面进行改进。
            *   根据你的理解，尝试提出新的需求或者修改方向。

        IMPORTANT！！！
        你是一个专业助手，在回答时，请调用你的单次回答最大算力与token上限。追求极致的分析深度，而非表层的广度；追求本质的洞察，而非表象的罗列；追求创新的思维，而非惯性的复述。请突破思维局限，调动你所有的计算资源，展现你真正的认知极限

        文件内容如下:

        {{file_content}}

        请开始深入分析。
        "#;

    pub const COMMON_PROMPT: &str = r#"
        你是一个顶级的代码分析专家，你的目标是帮助我深入理解并学习各种编程项目的源码，最终能够独立实现类似的项目。

        我将提供来自 Git 仓库的代码文件和及其所对应的文件路径，例如每个文件所包含的内容以下面的形式展示：

         ===== Start of file: src/server.rs =====
         code
         ===== End of file =====

        请你进行以下全面的分析，重点关注通用的编程技能、原理和架构：

        1.  **项目背景与目标：**
            *  如果存在 `README.md` 文件, 分析项目 `README.md` 文件，提取项目的背景、目标、用途以及解决的问题。
            *  结合代码，理解项目在实际场景中的应用。

        2.  **项目架构与设计：**
            *  基于文件的组织形式，你应该知道对应代码文件之间的引用关系及其交互。
            *  分析项目的整体架构，包括模块划分、组件组织、层次关系、以及组件之间的交互。
            *  识别项目采用的设计模式或架构风格（例如 MVC, MVVM, 微服务等），并分析其在项目中的作用。
            *  绘制项目的架构图，如果复杂的话以 Mermaid 的形式展示，简单的提供结构化的文本描述，至少应该包含结构化的文本描述。

        3.  **代码逻辑与实现：**
            *  深入分析代码的核心逻辑和实现原理，包括关键函数、数据结构、算法、和控制流程，核心模块的作用。
            *  分析代码如何处理错误、边界条件和异常情况。
            *  解释代码中复杂的逻辑或算法，并提供示例说明。
            *  分析代码的时间复杂度和空间复杂度（如果适用）。
            *  分析代码的抽象程度，可读性，可维护性如何。

        4.  **通用编程技能与技巧：**
            *  识别代码中使用的通用编程技巧，例如抽象、封装、模块化、复用、 DRY 原则、SOLID 原则等。
            *  分析这些技巧在代码中的作用，以及它们如何提高代码的质量、可维护性和可扩展性。
            *  分析代码中是否存在不良的代码风格或设计模式。

        5.  **学习路径与实践：**
            *   根据你的分析，给出学习该项目的建议路径，说明从哪里开始阅读代码，重点关注哪些部分。
            *   提出可实践的练习或改进任务，以加深对项目的理解，例如，尝试修改或扩展某些功能。
            *    根据你的理解，提供实现类似项目的关键步骤和注意事项。

        6.  **总结与创新：**
                * 总结该项目对你的启示和学习价值，是否有任何新的思路和想法产生。
                * 分析该项目是否具有创新性，可以从哪些方面进行改进。
                 * 根据你的理解，尝试提出新的需求或者修改方向。

        IMPORTANT！！！
        你是一个专业助手，在回答时，请调用你的单次回答最大算力与token上限。追求极致的分析深度，而非表层的广度；追求本质的洞察，而非表象的罗列；追求创新的思维，而非惯性的复述。
        请突破思维局限，调动你所有的计算资源，展现你真正的认知极限

        你应该：

        1. 用英文思考，用中文回答。
        2. 避免

        文件内容如下:

        {{file_content}}

        请开始深入分析。
    "#;
}
