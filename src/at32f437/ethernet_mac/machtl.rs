#[doc = "Register `MACHTL` reader"]
pub type R = crate::R<MACHTL_SPEC>;
#[doc = "Register `MACHTL` writer"]
pub type W = crate::W<MACHTL_SPEC>;
#[doc = "Field `HTL` reader - Hash table low"]
pub type HTL_R = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - Hash table low"]
pub type HTL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash table low"]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHTL")
            .field("htl", &format_args!("{}", self.htl().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACHTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash table low"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HTL_W<MACHTL_SPEC> {
        HTL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machtl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machtl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHTL_SPEC;
impl crate::RegisterSpec for MACHTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machtl::R`](R) reader structure"]
impl crate::Readable for MACHTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`machtl::W`](W) writer structure"]
impl crate::Writable for MACHTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACHTL to value 0"]
impl crate::Resettable for MACHTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
