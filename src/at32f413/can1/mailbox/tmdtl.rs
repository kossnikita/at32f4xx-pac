#[doc = "Register `TMDTL` reader"]
pub type R = crate::R<TMDTL_SPEC>;
#[doc = "Register `TMDTL` writer"]
pub type W = crate::W<TMDTL_SPEC>;
#[doc = "Field `TMDT[0-3]` reader - Transmit mailbox data byte %s"]
pub type TMDT_R = crate::FieldReader;
#[doc = "Field `TMDT[0-3]` writer - Transmit mailbox data byte %s"]
pub type TMDT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 8, O>;
impl R {
    #[doc = "Transmit mailbox data byte [0-3]"]
    #[inline(always)]
    pub unsafe fn tmdt(&self, n: u8) -> TMDT_R {
        TMDT_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Transmit mailbox data byte 0"]
    #[inline(always)]
    pub fn tmdt0(&self) -> TMDT_R {
        TMDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 1"]
    #[inline(always)]
    pub fn tmdt1(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 2"]
    #[inline(always)]
    pub fn tmdt2(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 3"]
    #[inline(always)]
    pub fn tmdt3(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMDTL")
            .field("tmdt0", &format_args!("{}", self.tmdt0().bits()))
            .field("tmdt1", &format_args!("{}", self.tmdt1().bits()))
            .field("tmdt2", &format_args!("{}", self.tmdt2().bits()))
            .field("tmdt3", &format_args!("{}", self.tmdt3().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TMDTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Transmit mailbox data byte [0-3]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn tmdt<const O: u8>(&mut self) -> TMDT_W<TMDTL_SPEC, O> {
        TMDT_W::new(self)
    }
    #[doc = "Bits 0:7 - Transmit mailbox data byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt0(&mut self) -> TMDT_W<TMDTL_SPEC, 0> {
        TMDT_W::new(self)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt1(&mut self) -> TMDT_W<TMDTL_SPEC, 8> {
        TMDT_W::new(self)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt2(&mut self) -> TMDT_W<TMDTL_SPEC, 16> {
        TMDT_W::new(self)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt3(&mut self) -> TMDT_W<TMDTL_SPEC, 24> {
        TMDT_W::new(self)
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
#[doc = "Transmit mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdtl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdtl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDTL_SPEC;
impl crate::RegisterSpec for TMDTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdtl::R`](R) reader structure"]
impl crate::Readable for TMDTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmdtl::W`](W) writer structure"]
impl crate::Writable for TMDTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMDTL to value 0"]
impl crate::Resettable for TMDTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
