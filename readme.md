rust
    compiling language
        rust code -> tokens -> ast -> hir(lowering 2018 edition) -> mir(lowering) -> llvm(optimize) -> binary(optimize)
        hir typechecking methodsearch
        mir marco genericstype skeleton(单态化)

    rust词法结构
        keywords
            strict(as/break/if/else/dyn...), 
            reserved(abstract/become/marco/override/priv/typeof...), 
            weak(union/static/dyc(2015edition))
        idendifiers
        comment
        whitespace
            \n \t tab
        tokens
            items
            block
            stmt
            expr
            pattern
            keyword
            ident
            literal
            vis
            punctuation
            delimeter
            token tree
            attributes
        path
            ::
            1.super::super::super::foo();
            2.
                struct t{}; impl t(){ fn f(){}; }   // t::f
                trait a1{ fn f(){} } impl a1 for t{} // <t as a1>::f()
            3.vec::<u8>::somefn() 

    expressions
        compile evaluation
            lisp/cpp
            ctfe
            macro + .rs
            rust ctfe
                const fn
                    const propagation -> prevent reuse so its like v8hotcode so it means a thing that has a const context is already determinable during compile time 
                const generic

    why rust non gc
        expr category
            position context
                variable init
                de -> *expr
                expr[expr]
                expr.field
                (expr)
        ownership
            当一个值包含堆分配内存（例如 String, Vec 等）时，它的所有权涉及到对堆上数据的管理。这种类型通常不能实现 Copy trait，因为直接复制它们会导致多次释放同一块堆内存的错误
        mutable
            let mut var1 = 1;

    数据类型
        基本数据类型
            number,string, float, reference, char, slice, pointer, tuple, unit, never, function pointer
            string
                string -> actual string store in heap
                str -> its a slice only a reference part of the string &str
            指针
                原始指针 *mut t / *const t
                nonnull rust recommendation to replace *mut t pointer, it also means its non null and respect the cycle
                function pointer -> means a pointer points to a function so u can use it directly its not point to a data
            reference    
                &t(不可变引用,) / &mut t(可变引用)
                引用意味着它拥有生命周期, 受检查其保护避免发生悬垂指针
            tuple   
                0个、1个或多个任意数据类型
            array
                [type: n] -> type / n = numbers of items
            
        自定义复合类型
        容器类型
        泛型
        特定类型
    
   