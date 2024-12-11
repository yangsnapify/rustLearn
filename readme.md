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
