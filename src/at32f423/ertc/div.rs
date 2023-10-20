#[doc = "Register `DIV` reader"]
pub type R = crate::R<DIV_SPEC>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DIV_SPEC>;
#[doc = "Field `DIVB` reader - Divider B"]
pub type DIVB_R = crate::FieldReader<u16>;
#[doc = "Field `DIVB` writer - Divider B"]
pub type DIVB_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `DIVA` reader - Divider A"]
pub type DIVA_R = crate::FieldReader;
#[doc = "Field `DIVA` writer - Divider A"]
pub type DIVA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:14 - Divider B"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - Divider A"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV")
            .field("diva", &format_args!("{}", self.diva().bits()))
            .field("divb", &format_args!("{}", self.divb().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:14 - Divider B"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DIVB_W<DIV_SPEC> {
        DIVB_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Divider A"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DIVA_W<DIV_SPEC> {
        DIVA_W::new(self, 16)
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
#[doc = "Divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV to value 0x007f_00ff"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x007f_00ff;
}
