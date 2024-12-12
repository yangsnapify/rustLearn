// 5. 综合应用题
// 5.1. 题目：创建一个泛型栈 Stack，支持推入元素、弹出元素和查看栈顶元素，且使用 where 子句约束栈元素必须实现 Clone 和 Debug。
// 5.2. 题目：实现一个函数 sum，它接收一个包含数字的切片并返回其和。要求函数只支持包含数字类型的切片，并且使用 where 子句约束。
// 5.3. 题目：为一个实现了 Clone trait 的泛型结构体添加一个 clone 方法，并确保该方法只能用于支持 Clone trait 的类型。
// 5.4. 题目：创建一个泛型类型 Container，它能存储一个任意类型的值，并为其实现 ToString trait。使用 where 子句约束容器中的类型必须实现 Display trait。
// 5.5. 题目：为一个类型 Rectangle 实现 area 方法，计算矩形的面积。矩形的宽度和高度是泛型类型，要求它们都必须实现 Copy 和 Debug trait。
// 5.6. 题目：实现一个函数 combine，接受两个泛型参数，返回它们的元组（tuple）。使用 where 子句确保这两个参数类型都支持 Clone 和 Debug。
// 5.7. 题目：创建一个泛型类型 Queue，它有 enqueue 和 dequeue 方法。确保它支持任意类型的元素，且元素类型实现了 Clone trait。