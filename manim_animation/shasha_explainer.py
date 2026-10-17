"""A visual explainer for Shasha's constrained code-generation boundary."""

from __future__ import annotations

from manim import (
    AnimationGroup,
    Arrow,
    BLACK,
    Circle,
    Create,
    CurvedArrow,
    DOWN,
    FadeIn,
    FadeOut,
    GrowFromCenter,
    LEFT,
    LaggedStart,
    Line,
    MovingCameraScene,
    ORIGIN,
    Rectangle,
    ReplacementTransform,
    RIGHT,
    RoundedRectangle,
    Succession,
    Text,
    Transform,
    TransformMatchingShapes,
    UP,
    VGroup,
    WHITE,
    Write,
    config,
    rate_functions,
)


# Product palette from web/styles.css.
PAPER = "#FFFDF8"
SOFT = "#F7F5F1"
SIDEBAR = "#F2EFEA"
INK = "#282321"
MUTED = "#6D6661"
SUBTLE = "#938B84"
LINE = "#DED9D2"
RED = "#9D1C20"
RED_DARK = "#731114"
RED_SOFT = "#F5E9E7"
DARK = "#221E1D"
GREEN = "#326C4B"
GREEN_SOFT = "#EAF1EC"
CODE = "#EEEAE5"
CODE_MUTED = "#A59E98"

SERIF = "Georgia"
SANS = "Arial"
MONO = "JetBrainsMonoNL Nerd Font Mono"

config.background_color = PAPER


def txt(
    value: str,
    size: float = 30,
    color: str = INK,
    font: str = SANS,
    weight: str = "NORMAL",
    line_spacing: float = 0.9,
) -> Text:
    return Text(
        value,
        font=font,
        font_size=size,
        color=color,
        weight=weight,
        line_spacing=line_spacing,
    )


def fit_width(mobject, maximum: float):
    if mobject.width > maximum:
        mobject.scale_to_fit_width(maximum)
    return mobject


def panel(width: float, height: float, fill: str = WHITE, stroke: str = LINE):
    return RoundedRectangle(
        width=width,
        height=height,
        corner_radius=0.08,
        fill_color=fill,
        fill_opacity=1,
        stroke_color=stroke,
        stroke_width=1.2,
    )


def tag(value: str, color: str = RED, fill: str = RED_SOFT, width: float | None = None):
    label = txt(value, 18, color, MONO, "MEDIUM")
    box_width = width or label.width + 0.38
    box = RoundedRectangle(
        width=box_width,
        height=0.48,
        corner_radius=0.05,
        stroke_color=color,
        stroke_width=1,
        fill_color=fill,
        fill_opacity=1,
    )
    return VGroup(box, label)


def brand(scale: float = 1.0):
    mark = Rectangle(
        width=0.48 * scale,
        height=0.48 * scale,
        fill_color=RED,
        fill_opacity=1,
        stroke_width=0,
    )
    letter = txt("S", 25 * scale, WHITE, SERIF, "BOLD").move_to(mark)
    name = txt("Shasha", 25 * scale, INK, SERIF, "BOLD")
    name.next_to(mark, RIGHT, buff=0.18 * scale)
    return VGroup(mark, letter, name)


def eyebrow(value: str):
    return txt(value.upper(), 16, RED, SANS, "BOLD").set_opacity(0.96)


def section_header(number: str, title: str, note: str | None = None):
    num = txt(number, 16, RED, MONO, "MEDIUM")
    heading = txt(title, 35, INK, SERIF, "BOLD")
    group = VGroup(num, heading).arrange(RIGHT, buff=0.28, aligned_edge=DOWN)
    if note:
        detail = txt(note, 17, MUTED)
        detail.next_to(group, RIGHT, buff=0.35, aligned_edge=DOWN)
        group.add(detail)
    return group


def code_block(lines: list[str], width: float, height: float, font_size: float = 19):
    background = panel(width, height, DARK, DARK)
    language = txt("RUST", 14, "#F08E81", MONO, "MEDIUM")
    label = txt("Rendered output", 15, CODE_MUTED)
    header = VGroup(language, label).arrange(RIGHT, buff=0.22)
    header.move_to(background.get_top() + DOWN * 0.35).align_to(background, LEFT).shift(RIGHT * 0.28)
    rule = Line(
        background.get_left() + RIGHT * 0.25 + UP * (height / 2 - 0.69),
        background.get_right() + LEFT * 0.25 + UP * (height / 2 - 0.69),
        color="#3F3936",
        stroke_width=1,
    )
    code_lines = VGroup()
    for index, line in enumerate(lines, 1):
        number = txt(f"{index:>2}", font_size - 5, "#6F6965", MONO)
        content = txt(line or " ", font_size, CODE, MONO)
        row = VGroup(number, content).arrange(RIGHT, buff=0.28, aligned_edge=UP)
        code_lines.add(row)
    code_lines.arrange(DOWN, buff=0.16, aligned_edge=LEFT)
    fit_width(code_lines, width - 0.55)
    code_lines.next_to(rule, DOWN, buff=0.3).align_to(background, LEFT).shift(RIGHT * 0.25)
    return VGroup(background, header, rule, code_lines)


def trace_row(need: str, choice: str, meaning: str, width: float = 5.5):
    base = Rectangle(
        width=width,
        height=0.92,
        stroke_color=LINE,
        stroke_width=1,
        fill_color=WHITE,
        fill_opacity=1,
    )
    check_box = Rectangle(width=0.32, height=0.32, stroke_color="#8EAA98", stroke_width=1)
    check = txt("✓", 17, GREEN, SANS, "BOLD").move_to(check_box)
    need_text = txt(need, 14, MUTED, MONO, "MEDIUM")
    meaning_text = txt(meaning, 15, INK)
    copy = VGroup(need_text, meaning_text).arrange(DOWN, buff=0.08, aligned_edge=LEFT)
    choice_text = txt(choice, 15, RED_DARK, MONO, "MEDIUM")
    fit_width(choice_text, 1.75)
    check_group = VGroup(check_box, check).move_to(base.get_left() + RIGHT * 0.35)
    copy.next_to(check_group, RIGHT, buff=0.2).align_to(base, UP).shift(DOWN * 0.17)
    choice_text.move_to(base.get_right() + LEFT * 1.0)
    return VGroup(base, check_group, copy, choice_text)


def flow_arrow(start, end, color: str = SUBTLE):
    return Arrow(start, end, buff=0.16, stroke_width=2, color=color, max_tip_length_to_length_ratio=0.12)


class ShashaExplainer(MovingCameraScene):
    """A 90-second, silent explainer designed for a 16:9 presentation."""

    def construct(self):
        self.camera.background_color = PAPER
        self.old_assumption()
        self.boundary_flip()
        self.inspectable_decision()
        self.composition_progression()
        self.controller_loop()
        self.second_pathway()
        self.final_reversal()

    def clear_stage(self, run_time: float = 0.6):
        if self.mobjects:
            self.play(FadeOut(VGroup(*self.mobjects), shift=UP * 0.08), run_time=run_time)

    def old_assumption(self):
        lockup = brand(0.88).to_corner(UP + LEFT, buff=0.45)
        label = eyebrow("The old assumption").move_to([-5.4, 2.55, 0], aligned_edge=LEFT)
        title = txt("Ask a model to write text.", 47, INK, SERIF, "BOLD")
        title.next_to(label, DOWN, buff=0.2, aligned_edge=LEFT)

        model_box = panel(2.3, 1.35, WHITE, INK).move_to([-4.6, 0.3, 0])
        model_label = txt("MODEL", 15, RED, MONO, "MEDIUM")
        model_copy = txt("predict every\ncharacter", 24, INK, SERIF, "BOLD", 0.8)
        model_group = VGroup(model_box, model_label, model_copy)
        model_label.move_to(model_box.get_top() + DOWN * 0.28)
        model_copy.move_to(model_box.get_center() + DOWN * 0.15)

        raw = code_block(
            ["fn calculate(a: i32, b: i32) {", "    a +", "}"],
            4.5,
            2.45,
            16,
        ).move_to([0.0, 0.3, 0])
        raw_label = tag("RAW TEXT", SUBTLE, SIDEBAR).next_to(raw, UP, buff=0.18)

        rejected_box = panel(2.35, 1.35, RED_SOFT, RED).move_to([4.75, 0.3, 0])
        cross = txt("×", 36, RED, SANS).move_to(rejected_box.get_left() + RIGHT * 0.42)
        rejected_label = txt("REJECTED", 14, RED, MONO, "BOLD")
        rejected_copy = txt("invalid\nprogram", 24, RED_DARK, SERIF, "BOLD", 0.8)
        rejected_label.move_to(rejected_box.get_top() + DOWN * 0.28).shift(RIGHT * 0.35)
        rejected_copy.move_to(rejected_box.get_center() + RIGHT * 0.35 + DOWN * 0.12)
        rejected = VGroup(rejected_box, cross, rejected_label, rejected_copy)

        first_arrow = flow_arrow(model_box.get_right(), raw.get_left(), RED)
        second_arrow = flow_arrow(raw.get_right(), rejected_box.get_left(), RED)

        self.play(FadeIn(lockup), FadeIn(label, shift=DOWN * 0.08), run_time=0.6)
        self.play(Write(title), run_time=0.9)
        self.play(FadeIn(model_group, shift=RIGHT * 0.2), run_time=0.6)
        self.play(Create(first_arrow), FadeIn(raw), FadeIn(raw_label), run_time=0.9)
        self.play(Create(second_arrow), FadeIn(rejected, shift=RIGHT * 0.15), run_time=0.8)

        repair_arrow = CurvedArrow(
            rejected_box.get_bottom() + DOWN * 0.1,
            model_box.get_bottom() + DOWN * 0.1,
            angle=-2.2,
            color=RED,
            stroke_width=2.5,
            tip_length=0.18,
        )
        repair_label = txt("DESCRIBE ERROR  ·  RE-PROMPT  ·  TRY AGAIN", 15, RED_DARK, MONO, "MEDIUM")
        repair_label.move_to([0.1, -2.05, 0])
        attempt_tags = VGroup(
            tag("ATTEMPT 01", RED, RED_SOFT),
            tag("ATTEMPT 02", RED, RED_SOFT),
            tag("ATTEMPT 03", RED, RED_SOFT),
        ).arrange(RIGHT, buff=0.12).next_to(repair_label, DOWN, buff=0.25)
        cost = txt("More repair attempts consume more time and compute.", 22, MUTED, SERIF)
        cost.to_edge(DOWN, buff=0.38)

        self.play(Create(repair_arrow), FadeIn(repair_label, shift=UP * 0.08), run_time=0.9)
        self.play(LaggedStart(*[FadeIn(item, shift=UP * 0.08) for item in attempt_tags], lag_ratio=0.2), run_time=1.1)
        self.play(FadeIn(cost), run_time=0.5)
        self.wait(1.2)
        self.clear_stage()

    def boundary_flip(self):
        overline = eyebrow("Shasha moves the boundary").move_to([0, 2.75, 0])
        question = txt("Who owns what?", 54, INK, SERIF, "BOLD").next_to(overline, DOWN, buff=0.22)
        divider = Line([0, 1.0, 0], [0, -2.25, 0], color=LINE, stroke_width=1.5)

        left_role = txt("THE MODEL", 16, RED, MONO, "BOLD")
        left_main = txt("chooses\nmeaning.", 48, RED_DARK, SERIF, "BOLD", 0.82)
        left_sub = txt("structured, legal actions", 19, MUTED)
        left_stack = VGroup(left_role, left_main, left_sub).arrange(DOWN, buff=0.2, aligned_edge=LEFT)
        left_stack.move_to([-3.45, -0.55, 0], aligned_edge=LEFT)

        right_role = txt("THE SYSTEM", 16, GREEN, MONO, "BOLD")
        right_main = txt("handles\nsyntax.", 48, INK, SERIF, "BOLD", 0.82)
        right_sub = txt("punctuation, braces, names, indentation", 19, MUTED)
        fit_width(right_sub, 5.5)
        right_stack = VGroup(right_role, right_main, right_sub).arrange(DOWN, buff=0.2, aligned_edge=LEFT)
        right_stack.move_to([1.15, -0.55, 0], aligned_edge=LEFT)

        action = tag("Add(@1, @2)", RED, RED_SOFT)
        action.move_to([-0.72, -2.75, 0])
        rust = tag("left + right", GREEN, GREEN_SOFT)
        rust.move_to([1.25, -2.75, 0])
        bridge = flow_arrow(action.get_right(), rust.get_left(), GREEN)

        self.play(FadeIn(overline), Write(question), run_time=0.8)
        self.play(Create(divider), run_time=0.5)
        self.play(
            FadeIn(left_stack, shift=RIGHT * 0.18),
            FadeIn(right_stack, shift=LEFT * 0.18),
            run_time=0.9,
        )
        self.play(FadeIn(action), Create(bridge), FadeIn(rust), run_time=0.9)
        self.wait(1.5)
        self.clear_stage()

    def inspectable_decision(self):
        header = section_header("01", "One decision, fully inspectable")
        header.to_edge(UP, buff=0.42).to_edge(LEFT, buff=0.55)
        prompt_box = Rectangle(
            width=13.1,
            height=0.72,
            stroke_width=0,
            fill_color=SIDEBAR,
            fill_opacity=1,
        ).next_to(header, DOWN, buff=0.35)
        prompt_label = txt("REQUEST", 14, SUBTLE, MONO, "BOLD")
        prompt = txt("Create a function that adds two integers", 22, INK, SANS, "MEDIUM")
        prompt_group = VGroup(prompt_label, prompt).arrange(RIGHT, buff=0.35)
        prompt_group.move_to(prompt_box).align_to(prompt_box, LEFT).shift(RIGHT * 0.28)

        left_title = txt("Function details", 24, INK, SERIF, "BOLD")
        left_note = txt("4 structured choices", 14, SUBTLE, MONO)
        left_heading = VGroup(left_title, left_note).arrange(RIGHT, buff=0.25, aligned_edge=DOWN)
        left_heading.move_to([-3.4, 1.55, 0])

        rows = VGroup(
            trace_row("Function kind", "Function", "Create one callable unit"),
            trace_row("Inputs", "I32 × 2", "Accept two integer values"),
            trace_row("Result", "I32", "Return one integer"),
            trace_row("Operation", "Add(@1, @2)", "Add the two input values"),
        ).arrange(DOWN, buff=0)
        rows.move_to([-3.55, -0.55, 0])

        code = code_block(
            [
                "fn calculate(left: i32, right: i32) -> i32 {",
                "    left + right",
                "}",
            ],
            6.4,
            4.35,
            17,
        ).move_to([3.35, -0.3, 0])

        footer_line = Line([-6.45, -3.15, 0], [6.45, -3.15, 0], color=LINE, stroke_width=1)
        owner_left = txt("CONTROLLER OWNS THE SHAPE", 14, RED, MONO, "BOLD")
        owner_right = txt("RENDERER OWNS THE SYNTAX", 14, GREEN, MONO, "BOLD")
        owner_left.next_to(footer_line, DOWN, buff=0.18).align_to(footer_line, LEFT)
        owner_right.next_to(footer_line, DOWN, buff=0.18).align_to(footer_line, RIGHT)

        self.play(FadeIn(header), FadeIn(prompt_box), FadeIn(prompt_group), run_time=0.7)
        self.play(FadeIn(left_heading), FadeIn(code[0]), FadeIn(code[1]), Create(code[2]), run_time=0.7)
        for index, row in enumerate(rows):
            code_line = code[3][index] if index < 3 else None
            animations = [FadeIn(row, shift=RIGHT * 0.12)]
            if code_line is not None:
                animations.append(FadeIn(code_line, shift=RIGHT * 0.08))
            self.play(*animations, run_time=0.45)
        self.play(Create(footer_line), FadeIn(owner_left), FadeIn(owner_right), run_time=0.7)
        self.wait(1.25)
        self.clear_stage()

    def composition_progression(self):
        header = section_header("02", "Compose structure", "Each request changes a few choices.")
        header.to_edge(UP, buff=0.42).to_edge(LEFT, buff=0.55)
        rule = Line([-6.55, 2.72, 0], [6.55, 2.72, 0], color=RED, stroke_width=2)

        steps = VGroup()
        step_data = [
            ("01", "ADDITION", "function + add"),
            ("02", "MULTIPLICATION", "tuple + multiply"),
            ("03", "OVERFLOW SAFETY", "optional + checked"),
        ]
        for number, title, detail in step_data:
            number_text = txt(number, 14, RED, MONO, "MEDIUM")
            title_text = txt(title, 15, INK, MONO, "BOLD")
            detail_text = txt(detail, 16, MUTED)
            stack = VGroup(number_text, title_text, detail_text).arrange(DOWN, buff=0.08, aligned_edge=LEFT)
            steps.add(stack)
        steps.arrange(RIGHT, buff=1.2, aligned_edge=UP)
        steps.move_to([0, 2.1, 0])

        progress_lines = VGroup()
        for left, right in zip(steps[:-1], steps[1:]):
            progress_lines.add(flow_arrow(left.get_right(), right.get_left(), SUBTLE))

        active_line = Rectangle(width=4.45, height=0.04, stroke_width=0, fill_color=RED, fill_opacity=1)
        active_line.next_to(steps[0], DOWN, buff=0.18)

        addition_tags = VGroup(
            tag("Function", RED, RED_SOFT),
            tag("I32", RED, RED_SOFT),
            tag("Add(@1, @2)", RED, RED_SOFT),
        ).arrange(RIGHT, buff=0.12)
        addition_tags.move_to([-3.25, 0.35, 0])
        addition_code = code_block(
            [
                "fn calculate(left: i32, right: i32) -> i32 {",
                "    left + right",
                "}",
            ],
            6.4,
            3.65,
            16,
        ).move_to([3.25, -0.35, 0])

        meaning_title = txt("STRUCTURED MEANING", 14, RED, MONO, "BOLD")
        meaning_title.next_to(addition_tags, UP, buff=0.28).align_to(addition_tags, LEFT)
        explain = txt("One operation. One integer result.", 25, INK, SERIF, "BOLD")
        explain.next_to(addition_tags, DOWN, buff=0.38).align_to(addition_tags, LEFT)
        request = txt("“Create a function that adds two integers.”", 19, MUTED, SERIF)
        request.next_to(explain, DOWN, buff=0.2).align_to(explain, LEFT)
        left_group = VGroup(meaning_title, addition_tags, explain, request)

        self.play(FadeIn(header), Create(rule), run_time=0.6)
        self.play(
            LaggedStart(*[FadeIn(step, shift=UP * 0.08) for step in steps], lag_ratio=0.16),
            LaggedStart(*[Create(line) for line in progress_lines], lag_ratio=0.25),
            run_time=1.0,
        )
        self.play(FadeIn(active_line), FadeIn(left_group), FadeIn(addition_code), run_time=0.8)
        self.wait(0.9)

        multiply_tags = VGroup(
            tag("Tuple<I32, I32>", RED, RED_SOFT),
            tag("Add", RED, RED_SOFT),
            tag("Mul", RED, RED_SOFT),
        ).arrange(RIGHT, buff=0.12).move_to(addition_tags)
        multiply_explain = txt("Two results reuse the same inputs.", 25, INK, SERIF, "BOLD").move_to(explain, aligned_edge=LEFT)
        multiply_request = txt("“Please now add multiplication.”", 19, MUTED, SERIF).move_to(request, aligned_edge=LEFT)
        multiply_code = code_block(
            [
                "fn calculate(left: i32, right: i32) -> (i32, i32) {",
                "    (left + right, left * right)",
                "}",
            ],
            6.4,
            3.65,
            16,
        ).move_to(addition_code)
        active_line_2 = active_line.copy().next_to(steps[1], DOWN, buff=0.18)
        self.play(
            Transform(active_line, active_line_2),
            TransformMatchingShapes(addition_tags, multiply_tags),
            ReplacementTransform(explain, multiply_explain),
            ReplacementTransform(request, multiply_request),
            ReplacementTransform(addition_code, multiply_code),
            run_time=1.1,
        )
        self.wait(1.0)

        checked_tags = VGroup(
            tag("Option<Tuple>", RED, RED_SOFT),
            tag("checked_add", GREEN, GREEN_SOFT),
            tag("checked_mul", GREEN, GREEN_SOFT),
        ).arrange(RIGHT, buff=0.12).move_to(multiply_tags)
        checked_explain = txt("Failure becomes explicit in the result.", 25, INK, SERIF, "BOLD").move_to(multiply_explain, aligned_edge=LEFT)
        checked_request = txt("“Prevent integer overflow.”", 19, MUTED, SERIF).move_to(multiply_request, aligned_edge=LEFT)
        checked_code = code_block(
            [
                "fn calculate(left: i32, right: i32)",
                "    -> Option<(i32, i32)> {",
                "    let sum = left.checked_add(right)?;",
                "    let product = left.checked_mul(right)?;",
                "    Some((sum, product))",
                "}",
            ],
            6.4,
            4.7,
            15,
        ).move_to([3.25, -0.55, 0])
        active_line_3 = active_line.copy().next_to(steps[2], DOWN, buff=0.18)
        self.play(
            Transform(active_line, active_line_3),
            TransformMatchingShapes(multiply_tags, checked_tags),
            ReplacementTransform(multiply_explain, checked_explain),
            ReplacementTransform(multiply_request, checked_request),
            ReplacementTransform(multiply_code, checked_code),
            run_time=1.2,
        )
        statement = txt("The answer grows by legal choices — not by rewriting a blob of text.", 21, RED_DARK, SERIF, "BOLD")
        statement.to_edge(DOWN, buff=0.32)
        self.play(FadeIn(statement, shift=UP * 0.08), run_time=0.6)
        self.wait(1.6)
        self.clear_stage()

    def controller_loop(self):
        header = section_header("03", "The controller is the source of truth")
        header.to_edge(UP, buff=0.42).to_edge(LEFT, buff=0.55)
        subhead = txt("The model may score every candidate. Only legal actions survive.", 20, MUTED)
        subhead.next_to(header, DOWN, buff=0.17, aligned_edge=LEFT)

        table_box = panel(6.25, 4.45, WHITE, INK).move_to([-3.45, -0.45, 0])
        table_label = txt("MODEL SCORES", 14, RED, MONO, "BOLD")
        table_label.next_to(table_box.get_top(), DOWN, buff=0.28).align_to(table_box, LEFT).shift(RIGHT * 0.28)

        candidates = [
            ("ReturnString", 0.91, False),
            ("ReturnTuple", 0.76, True),
            ("ReturnInteger", 0.63, True),
            ("OpenBlock", 0.28, False),
        ]
        candidate_rows = VGroup()
        for name, score, _legal in candidates:
            name_text = txt(name, 17, INK, MONO, "MEDIUM")
            bar_bg = Rectangle(width=2.4, height=0.18, stroke_width=0, fill_color=SIDEBAR, fill_opacity=1)
            bar = Rectangle(width=2.4 * score, height=0.18, stroke_width=0, fill_color=RED, fill_opacity=1)
            bar.align_to(bar_bg, LEFT)
            score_text = txt(f"{score:.2f}", 15, MUTED, MONO)
            bar_group = VGroup(bar_bg, bar)
            row = VGroup(name_text, bar_group, score_text)
            name_text.move_to([-5.75, 0, 0], aligned_edge=LEFT)
            bar_group.move_to([-3.05, 0, 0], aligned_edge=LEFT)
            score_text.move_to([-1.0, 0, 0])
            candidate_rows.add(row)
        candidate_rows.arrange(DOWN, buff=0.55, center=False)
        candidate_rows.move_to([-3.45, -0.45, 0])

        mask_title = txt("LEGALITY MASK", 14, RED, MONO, "BOLD")
        mask_title.move_to([1.0, 1.52, 0], aligned_edge=LEFT)
        mask_note = txt("Current state permits\ninteger-shaped results only.", 21, INK, SERIF, "BOLD", 0.9)
        mask_note.next_to(mask_title, DOWN, buff=0.25, aligned_edge=LEFT)

        verdicts = VGroup()
        for _name, _score, legal in candidates:
            word = "LEGAL" if legal else "MASKED"
            color = GREEN if legal else RED
            fill = GREEN_SOFT if legal else RED_SOFT
            verdicts.add(tag(word, color, fill, 1.5))
        verdicts.arrange(DOWN, buff=0.46)
        verdicts.move_to([1.7, -0.76, 0])

        selected = panel(3.25, 1.25, GREEN_SOFT, GREEN).move_to([4.7, -0.62, 0])
        selected_label = txt("CHOOSE BEST LEGAL", 13, GREEN, MONO, "BOLD")
        selected_action = txt("ReturnTuple", 22, INK, MONO, "BOLD")
        selected_label.move_to(selected.get_top() + DOWN * 0.27)
        selected_action.move_to(selected.get_center() + DOWN * 0.15)
        selected_group = VGroup(selected, selected_label, selected_action)
        selection_arrow = flow_arrow(verdicts.get_right(), selected.get_left(), GREEN)

        self.play(FadeIn(header), FadeIn(subhead), run_time=0.7)
        self.play(FadeIn(table_box), FadeIn(table_label), run_time=0.5)
        self.play(LaggedStart(*[FadeIn(row, shift=RIGHT * 0.12) for row in candidate_rows], lag_ratio=0.18), run_time=1.2)
        self.play(FadeIn(mask_title), FadeIn(mask_note), run_time=0.6)
        self.play(LaggedStart(*[FadeIn(item) for item in verdicts], lag_ratio=0.15), run_time=1.0)
        self.play(Create(selection_arrow), FadeIn(selected_group, shift=RIGHT * 0.15), run_time=0.8)
        self.wait(0.9)

        flow_labels = ["SCORE", "MASK", "CHOOSE", "APPLY", "RENDER"]
        flow_nodes = VGroup()
        for index, value in enumerate(flow_labels):
            fill = RED_SOFT if index < 4 else GREEN_SOFT
            stroke = RED if index < 4 else GREEN
            flow_nodes.add(tag(value, stroke, fill, 1.75))
        flow_nodes.arrange(RIGHT, buff=0.62).to_edge(DOWN, buff=0.46)
        flow_arrows = VGroup(
            *[
                flow_arrow(left.get_right(), right.get_left(), SUBTLE)
                for left, right in zip(flow_nodes[:-1], flow_nodes[1:])
            ]
        )
        self.play(
            LaggedStart(*[FadeIn(node, shift=UP * 0.08) for node in flow_nodes], lag_ratio=0.12),
            LaggedStart(*[Create(arrow) for arrow in flow_arrows], lag_ratio=0.12),
            run_time=1.2,
        )
        self.wait(1.5)
        self.clear_stage()

    def second_pathway(self):
        header = section_header("04", "Same mechanism, different pathway")
        header.to_edge(UP, buff=0.42).to_edge(LEFT, buff=0.55)
        route = eyebrow("Search a collection").next_to(header, DOWN, buff=0.22, aligned_edge=LEFT)

        input_label = txt("INPUT", 14, SUBTLE, MONO, "BOLD")
        values = [7, 4, 2, 4, 9, 6]
        value_boxes = VGroup()
        for value in values:
            box = Rectangle(width=0.72, height=0.72, stroke_color=LINE, stroke_width=1, fill_color=WHITE, fill_opacity=1)
            number = txt(str(value), 24, INK, MONO, "MEDIUM").move_to(box)
            value_boxes.add(VGroup(box, number))
        value_boxes.arrange(RIGHT, buff=0.08)
        input_group = VGroup(input_label, value_boxes).arrange(DOWN, buff=0.2, aligned_edge=LEFT)
        input_group.move_to([-3.7, 1.35, 0])

        stage_data = [
            ("Find", "first even", "4", "Option<I32>"),
            ("Filter", "every even", "[4, 2, 4, 6]", "Vec<I32>"),
            ("Filter + Set", "unique even", "[4, 2, 6]", "Vec<I32>"),
        ]
        stage_groups = VGroup()
        for action, meaning, result, shape in stage_data:
            action_tag = tag(action, RED, RED_SOFT)
            copy = txt(meaning, 21, INK, SERIF, "BOLD")
            arrow = txt("→", 26, SUBTLE, SANS)
            result_text = txt(result, 21, GREEN, MONO, "BOLD")
            shape_tag = tag(shape, GREEN, GREEN_SOFT)
            row = VGroup(action_tag, copy, arrow, result_text, shape_tag).arrange(RIGHT, buff=0.25)
            stage_groups.add(row)
        stage_groups.arrange(DOWN, buff=0.48, aligned_edge=LEFT)
        fit_width(stage_groups, 12.6)
        stage_groups.move_to([0, -0.65, 0], aligned_edge=LEFT)

        spine = Line([-5.95, 0.35, 0], [-5.95, -2.35, 0], color=RED, stroke_width=2)
        dots = VGroup(*[Circle(0.07, color=RED, fill_color=RED, fill_opacity=1) for _ in range(3)])
        for dot, row in zip(dots, stage_groups):
            dot.move_to([spine.get_x(), row.get_y(), 0])
            row.shift(RIGHT * 0.55)

        proof = txt("Authored route. Reproducible clicks. The same structured representation.", 22, MUTED, SERIF)
        proof.to_edge(DOWN, buff=0.42)

        self.play(FadeIn(header), FadeIn(route), FadeIn(input_group, shift=RIGHT * 0.15), run_time=0.8)
        self.play(Create(spine), run_time=0.5)
        for dot, row in zip(dots, stage_groups):
            self.play(GrowFromCenter(dot), FadeIn(row, shift=RIGHT * 0.18), run_time=0.65)
        self.play(FadeIn(proof), run_time=0.6)
        self.wait(1.5)
        self.clear_stage()

    def final_reversal(self):
        lockup = brand(1.05).move_to([0, 2.75, 0])
        overline = eyebrow("The reversal").next_to(lockup, DOWN, buff=0.55)

        line_one = txt("The model is responsible", 47, INK, SERIF, "BOLD")
        line_one.move_to([0, 0.65, 0])
        line_two = txt("for meaning.", 58, RED_DARK, SERIF, "BOLD")
        line_two.next_to(line_one, DOWN, buff=0.05)
        line_three = txt("Correctness belongs to", 47, INK, SERIF, "BOLD")
        line_three.next_to(line_two, DOWN, buff=0.34)
        line_four = txt("the system around it.", 58, GREEN, SERIF, "BOLD")
        line_four.next_to(line_three, DOWN, buff=0.05)

        rule = Line([-5.0, -2.5, 0], [5.0, -2.5, 0], color=LINE, stroke_width=1)
        honest_scope = txt(
            "GUIDED UI: DETERMINISTIC  ·  RUST PROTOTYPE: SCORE → MASK → CHOOSE → APPLY → RENDER",
            14,
            MUTED,
            MONO,
            "MEDIUM",
        ).next_to(rule, DOWN, buff=0.22)

        self.play(FadeIn(lockup, shift=DOWN * 0.08), FadeIn(overline), run_time=0.8)
        self.play(Write(line_one), Write(line_two), run_time=1.2)
        self.play(FadeIn(line_three, shift=UP * 0.08), FadeIn(line_four, shift=UP * 0.08), run_time=1.0)
        self.play(Create(rule), FadeIn(honest_scope), run_time=0.7)
        self.wait(3.0)

