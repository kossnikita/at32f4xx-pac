#[doc = "Register `REMAP2` writer"]
pub type W = crate::W<REMAP2_SPEC>;
#[doc = "CMP internal muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP_MUX_A {
    #[doc = "0: CMP1_OUT is connected to PA0, CMP2_OUT is connected to PA2"]
    Mux0 = 0,
    #[doc = "1: CMP1_OUT is connected to PA6, CMP2_OUT is connected to PA7"]
    Mux1 = 1,
    #[doc = "2: CMP1_OUT is connected to PA11, CMP2_OUT is connected to PA12;"]
    Mux2 = 2,
}
impl From<CMP_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMP_MUX_A {
    type Ux = u8;
}
#[doc = "Field `CMP_MUX` writer - CMP internal muxing"]
pub type CMP_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CMP_MUX_A>;
impl<'a, REG> CMP_MUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP1_OUT is connected to PA0, CMP2_OUT is connected to PA2"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(CMP_MUX_A::Mux0)
    }
    #[doc = "CMP1_OUT is connected to PA6, CMP2_OUT is connected to PA7"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(CMP_MUX_A::Mux1)
    }
    #[doc = "CMP1_OUT is connected to PA11, CMP2_OUT is connected to PA12;"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(CMP_MUX_A::Mux2)
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 26:27 - CMP internal muxing"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_mux(&mut self) -> CMP_MUX_W<REMAP2_SPEC> {
        CMP_MUX_W::new(self, 26)
    }
}
#[doc = "IO MUX remap register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP2_SPEC;
impl crate::RegisterSpec for REMAP2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`remap2::W`](W) writer structure"]
impl crate::Writable for REMAP2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP2 to value 0"]
impl crate::Resettable for REMAP2_SPEC {
    const RESET_VALUE: u32 = 0;
}
